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
const {Manifest} = require('release-please');
const {Version} = require('release-please/build/src/version');
const {updateReleasePullRequest} = require('./release-please-pr.cjs');
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
assert.equal(config['skip-github-release'], true);
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
assert.equal(job.steps.length, 3);
assert.match(job.steps[0].uses, /^actions\/checkout@[0-9a-f]{40}$/);
assert.deepEqual(job.steps[0].with, {'persist-credentials': false});
assert(job.steps[1].run.includes('release-please@17.3.0'));
assert(job.steps[1].run.includes('--ignore-scripts'));
assert.deepEqual(job.steps[2].env, {GITHUB_TOKEN: '${{ secrets.GITHUB_TOKEN }}'});
assert(job.steps[2].run.includes('node scripts/release-please-pr.cjs'));
assert(!job.steps.some(step => /github-release|cargo publish|git tag/.test(step.run || '')));

const publishing = YAML.parse(read('.github/workflows/publish-crates.yml'));
assert.deepEqual(Object.keys(publishing.on).sort(), ['push', 'workflow_dispatch']);
assert.deepEqual(publishing.on.push.branches, ['main']);
assert.deepEqual(publishing.on.push.paths, ['.release-please-manifest.json']);
assert.deepEqual(publishing.permissions, {contents: 'read'});
assert.equal(publishing.concurrency['cancel-in-progress'], false);
const preparation = publishing.jobs.prepare;
assert.equal(preparation.if, "github.ref == 'refs/heads/main'");
assert.deepEqual(preparation.permissions, {contents: 'read', 'pull-requests': 'read'});
assert.equal(preparation.steps[0].with['fetch-depth'], 0);
assert.equal(preparation.steps[0].with['persist-credentials'], false);
const publication = publishing.jobs.publish;
assert.equal(publication.needs, 'prepare');
assert.equal(publication.if, "needs.prepare.outputs.ready == 'true'");
assert.deepEqual(publication.permissions, {contents: 'write'});
assert.equal(publication.steps[0].with.ref, '${{ needs.prepare.outputs.sha }}');
for (const step of [...preparation.steps, ...publication.steps]) {
  if (step.uses) assert.match(step.uses, /^actions\/checkout@[0-9a-f]{40}$/);
}
const upload = publication.steps.at(-1);
assert.equal(upload.env.CARGO_REGISTRY_TOKEN, '${{ secrets.CARGO_REGISTRY_TOKEN }}');
assert.equal(upload.env.GH_TOKEN, '${{ secrets.GITHUB_TOKEN }}');
assert(upload.run.includes('scripts/publish-crates.py publish'));
assert(!publication.steps.slice(0, -1).some(step => JSON.stringify(step).includes('CARGO_REGISTRY_TOKEN')));

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
    skipGithubRelease: config['skip-github-release'],
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
  assert.equal(release, undefined, 'A merged PR must not generate a GitHub Release or tag');
  console.log(`PASS: ${previousVersion || 'initial'} + ${message} -> ${expectedVersion}; all five crates, seven dependencies and lockfile match; no release or tag`);
  return pr;
}

async function taglessLifecycle(initialPr) {
  const boundary = 'b'.repeat(40);
  const merged = {
    number: 10,
    title: initialPr.title.toString(),
    body: initialPr.body.toString(),
    headBranchName: initialPr.headRefName,
    baseBranchName: 'main',
    sha: boundary,
    labels: ['autorelease: pending'],
    files: [],
  };
  let history = [
    {sha: 'c'.repeat(40), message: 'fix: fresh fix', files: ['crates/core/src/lib.rs']},
    {sha: boundary, message: 'chore: release 0.1.0', files: ['.release-please-manifest.json'], pullRequest: merged},
    {sha, message: 'feat: old SDK feature', files: ['crates/core/src/lib.rs']},
  ];
  let removed = 0;
  let created = 0;
  let baseline;
  const mock = {
    ...github,
    async getFileJson(path, branch) {
      assert.equal(path, '.release-please-manifest.json');
      assert.equal(branch, 'main');
      return {'.': '0.1.0'};
    },
    async *mergeCommitIterator(branch) {
      assert.equal(branch, 'main');
      yield* history;
    },
    async *releaseIterator() {},
    async *tagIterator() {},
    async *pullRequestIterator(branch, state) {
      assert.equal(branch, 'main');
      if (state === 'MERGED') yield merged;
    },
    async removeIssueLabels(labels, number) {
      assert.deepEqual(labels, ['autorelease: pending']);
      assert.equal(number, 10);
      merged.labels = [];
      removed += 1;
    },
    async createPullRequest(pr, target, message, updates) {
      assert.equal(target, 'main');
      assert.equal(removed, 1, 'The merged PR must not block its successor');
      assert(pr.body.includes('fresh fix'));
      assert(!pr.body.includes('old SDK feature'));
      assert(!pr.body.includes('initial SDK'));
      const cargo = updates.find(update => update.path === 'Cargo.toml');
      assert.equal(parseCargoManifest(cargo.updater.updateContent(read('Cargo.toml'))).package.version, '0.1.1');
      created += 1;
      return {...pr, number: 11};
    },
    async createRelease() { assert.fail('GitHub release creation is forbidden'); },
    async createTag() { assert.fail('Tag creation is forbidden'); },
  };
  const createManifest = async options => {
    baseline = options.lastReleaseSha;
    return new Manifest(mock, 'main', {'.': {
      releaseType: 'rust',
      skipGithubRelease: true,
      includeComponentInTag: false,
      includeVInTag: true,
      bumpMinorPreMajor: true,
      bumpPatchForMinorPreMajor: false,
    }}, {'.': Version.parse('0.1.0')}, options);
  };
  const result = await updateReleasePullRequest(mock, createManifest);
  assert.equal(baseline, boundary);
  assert.equal(result.length, 1);
  assert.equal(created, 1);
  assert.equal(removed, 1);
  console.log('PASS: merged version PR without a tag -> fresh patch PR; old commits excluded');

  history = history.slice(1);
  assert.deepEqual(await updateReleasePullRequest(mock, createManifest), []);
  assert.equal(created, 1);
  assert.equal(removed, 1);
  console.log('PASS: no-change rerun -> no duplicate version PR or label change');

  history = [{sha, message: 'fix: unrelated change', files: ['README.md']}];
  await assert.rejects(updateReleasePullRequest(mock, createManifest), /Cannot find the version manifest commit/);
  assert.equal(created, 1);
  assert.equal(removed, 1);
  console.log('PASS: missing version boundary fails closed before any write');

  let initialOptions;
  const initialMock = {
    ...github,
    async getFileJson() { return {}; },
    async *mergeCommitIterator() { assert.fail('An initial run has no version boundary'); },
  };
  const fakeManifest = {
    async *findMergedReleasePullRequests() {},
    async createPullRequests() { return []; },
  };
  await updateReleasePullRequest(initialMock, async options => {
    initialOptions = options;
    return fakeManifest;
  });
  assert.equal(initialOptions.lastReleaseSha, undefined);
  console.log('PASS: initial empty manifest preserves the first-release bootstrap');
}

async function main() {
  const initialPr = await scenario('feat: initial SDK', undefined, '0.1.0');
  await scenario('fix: correct request encoding', '0.1.0', '0.1.1');
  await scenario('feat: add another operation', '0.1.0', '0.2.0');
  await scenario('feat!: change the client API', '0.1.0', '0.2.0');
  await scenario('feat: add another operation', '1.0.0', '1.1.0');
  await scenario('feat!: change the client API', '1.0.0', '2.0.0');
  await scenario('ci: maintain release automation', '0.1.0', undefined);
  await taglessLifecycle(initialPr);
  console.log('PASS: config schema, workflow safety, seven version scenarios and tagless lifecycle checks');
}
main().catch(error => {
  console.error(error);
  process.exitCode = 1;
});
