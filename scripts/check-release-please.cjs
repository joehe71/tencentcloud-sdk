#!/usr/bin/env node
'use strict';

// Local-only release simulation. No GitHub client or network transport is created.
const assert = require('node:assert/strict');
const {readFileSync} = require('node:fs');
const {resolve} = require('node:path');
const Ajv = require('ajv');
const addFormats = require('ajv-formats');
const YAML = require('yaml');
const {buildStrategy} = require('release-please/build/src/factory');
const {parseConventionalCommits} = require('release-please/build/src/commit');
const {TagName} = require('release-please/build/src/util/tag-name');
const {parseCargoManifest, parseCargoLockfile} = require('release-please/build/src/updaters/rust/common');

const root = resolve(__dirname, '..');
const read = path => readFileSync(resolve(root, path), 'utf8');
const config = JSON.parse(read('release-please-config.json'));
const manifest = JSON.parse(read('.release-please-manifest.json'));
const schema = require('release-please/schemas/config.json');
const ajv = new Ajv({allErrors: true, strict: false});
addFormats(ajv);
assert(ajv.validate(schema, config), JSON.stringify(ajv.errors));
assert.equal(config['release-type'], 'rust');
assert.deepEqual(config.packages, {'.': {}});
assert.equal(config['include-component-in-tag'], false);
assert.equal(config['include-v-in-tag'], true);
assert.equal(config['bump-minor-pre-major'], true);
assert.equal(config['bump-patch-for-minor-pre-major'], false);
assert.equal(config['release-as'], undefined);
assert.deepEqual(Object.keys(manifest).filter(path => path !== '.'), []);
for (const version of Object.values(manifest)) {
  assert.match(version, /^\d+\.\d+\.\d+$/);
}

const workflow = YAML.parse(read('.github/workflows/release-please.yml'));
assert.deepEqual(Object.keys(workflow.on).sort(), ['push', 'workflow_dispatch']);
assert.deepEqual(workflow.on.push.branches, ['main']);
assert.equal(workflow.concurrency['cancel-in-progress'], false);
assert.deepEqual(workflow.permissions, {contents: 'read'});
const job = workflow.jobs['release-please'];
assert.equal(job.if, "github.ref == 'refs/heads/main'");
assert.deepEqual(job.permissions, {contents: 'write', issues: 'write', 'pull-requests': 'write'});
assert.equal(job.steps.length, 1);
assert.match(job.steps[0].uses, /^googleapis\/release-please-action@[0-9a-f]{40}$/);
assert.deepEqual(job.steps[0].with, {
  token: '${{ secrets.GITHUB_TOKEN }}',
  'target-branch': 'main',
  'config-file': 'release-please-config.json',
  'manifest-file': '.release-please-manifest.json',
});

const workspace = parseCargoManifest(read('Cargo.toml'));
const paths = ['Cargo.toml', ...workspace.workspace.members.map(member => `${member}/Cargo.toml`)];
const names = new Set(paths.map(path => parseCargoManifest(read(path)).package.name));
assert.equal(names.size, 5);
const originalFiles = new Map([...paths, 'Cargo.lock', 'CHANGELOG.md'].map(path => [path, read(path)]));
const sha = 'a'.repeat(40);
const quietLogger = {info() {}, warn() {}, error() {}, debug() {}};
const github = {
  repository: {owner: 'example', repo: 'sdk', defaultBranch: 'main'},
  async getFileContentsOnBranch(path, branch) {
    assert.equal(branch, 'main');
    assert(originalFiles.has(path), `Unexpected file lookup: ${path}`);
    const parsedContent = originalFiles.get(path);
    return {parsedContent, content: Buffer.from(parsedContent).toString('base64'), sha};
  },
};

async function scenario(message, previousVersion, expectedVersion) {
  const strategy = await buildStrategy({
    github,
    releaseType: config['release-type'],
    path: '.',
    targetBranch: 'main',
    includeComponentInTag: config['include-component-in-tag'],
    includeVInTag: config['include-v-in-tag'],
    bumpMinorPreMajor: config['bump-minor-pre-major'],
    bumpPatchForMinorPreMajor: config['bump-patch-for-minor-pre-major'],
    logger: quietLogger,
  });
  const commits = parseConventionalCommits([{sha, message, files: ['crates/core/src/lib.rs']}]);
  const previousRelease = previousVersion
    ? {tag: TagName.parse(`v${previousVersion}`), sha: 'b'.repeat(40), notes: 'Previous release'}
    : undefined;
  const pr = await strategy.buildReleasePullRequest(commits, previousRelease);
  if (expectedVersion === undefined) {
    assert.equal(pr, undefined);
    console.log(`PASS: ${message} -> no release PR`);
    return;
  }
  assert(pr, `No release PR for ${message}`);
  assert.equal(pr.version.toString(), expectedVersion);
  assert.deepEqual(new Set(pr.updates.map(update => update.path)), new Set(originalFiles.keys()));
  const updatedFiles = new Map(pr.updates.map(update => [
    update.path, update.updater.updateContent(originalFiles.get(update.path), quietLogger),
  ]));
  let dependencyCount = 0;
  for (const path of paths) {
    const cargo = parseCargoManifest(updatedFiles.get(path));
    assert.equal(cargo.package.version, expectedVersion, path);
    assert.deepEqual(cargo.package.edition, {workspace: true});
    assert.deepEqual(cargo.package['rust-version'], {workspace: true});
    for (const kind of ['dependencies', 'dev-dependencies', 'build-dependencies']) {
      for (const [name, dependency] of Object.entries(cargo[kind] || {})) {
        if (names.has(name) && dependency.path) {
          assert.equal(dependency.version, expectedVersion, `${path}: ${name}`);
          dependencyCount += 1;
        }
      }
    }
  }
  assert.equal(dependencyCount, 7);
  const beforeLock = parseCargoLockfile(originalFiles.get('Cargo.lock'));
  const afterLock = parseCargoLockfile(updatedFiles.get('Cargo.lock'));
  const localPackages = afterLock.package.filter(pkg => names.has(pkg.name));
  assert.equal(localPackages.length, 5);
  for (const pkg of localPackages) assert.equal(pkg.version, expectedVersion, pkg.name);
  assert.deepEqual(
    afterLock.package.filter(pkg => !names.has(pkg.name)),
    beforeLock.package.filter(pkg => !names.has(pkg.name)),
  );
  assert(updatedFiles.get('CHANGELOG.md').includes(expectedVersion));
  const release = await strategy.buildRelease({
    title: pr.title.toString(),
    body: pr.body.toString(),
    headBranchName: pr.headRefName,
    baseBranchName: 'main',
    number: 1,
    labels: [],
    files: [],
    sha,
  });
  assert(release, 'No release generated from the simulated merged PR');
  assert.equal(release.tag.toString(), `v${expectedVersion}`);
  assert.equal(release.sha, sha);
  assert(release.notes.includes(message.split(': ').slice(1).join(': ')));
  console.log(`PASS: ${previousVersion || 'initial'} + ${message} -> v${expectedVersion}; all five crates, seven dependencies, lockfile and release notes match`);
}

async function main() {
  await scenario('feat: initial SDK', undefined, '0.1.0');
  await scenario('fix: correct request encoding', '0.1.0', '0.1.1');
  await scenario('feat: add another operation', '0.1.0', '0.2.0');
  await scenario('feat!: change the client API', '0.1.0', '0.2.0');
  await scenario('feat: add another operation', '1.0.0', '1.1.0');
  await scenario('feat!: change the client API', '1.0.0', '2.0.0');
  await scenario('ci: maintain release automation', '0.1.0', undefined);
  console.log('PASS: config schema, workflow safety and seven offline release scenarios');
}
main().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
