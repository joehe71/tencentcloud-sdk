#!/usr/bin/env node
'use strict';

const {GitHub, Manifest} = require('release-please');
const {Version} = require('release-please/build/src/version');

const branch = 'main';
const manifestPath = '.release-please-manifest.json';

// No createReleases(), createRelease(), or Git tag operation is called here.
async function updateReleasePullRequest(github, createManifest = options =>
  Manifest.fromManifest(github, branch, 'release-please-config.json', manifestPath, options)) {
  const versions = await github.getFileJson(manifestPath, branch);
  let lastReleaseSha;
  if (versions['.']) {
    Version.parse(versions['.']);
    // A merged version PR is the boundary, not a tag or GitHub Release.
    for await (const commit of github.mergeCommitIterator(branch, {
      maxResults: 500,
      backfillFiles: true,
    })) {
      if (commit.files.includes(manifestPath)) {
        lastReleaseSha = commit.sha;
        break;
      }
    }
    if (!lastReleaseSha) {
      throw new Error('Cannot find the version manifest commit within the search limit');
    }
  }
  const manifest = await createManifest({lastReleaseSha});
  // Release Please otherwise blocks new PRs while a merged PR is still pending a tag.
  for await (const pr of manifest.findMergedReleasePullRequests()) {
    if (pr.baseBranchName !== branch ||
        !/^release-please--branches--main(?:--components--tencentcloud-sdk)?$/.test(pr.headBranchName)) {
      throw new Error('Unexpected pending release PR; refusing to change its labels');
    }
    if (!lastReleaseSha) {
      throw new Error('A pending merged release PR has no version manifest boundary');
    }
    await github.removeIssueLabels(manifest.labels, pr.number);
  }
  return manifest.createPullRequests();
}

async function main() {
  if (process.env.GITHUB_REF !== 'refs/heads/main') {
    throw new Error('Release PR automation must run against main');
  }
  const repository = process.env.GITHUB_REPOSITORY || '';
  if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(repository) || !process.env.GITHUB_TOKEN) {
    throw new Error('Missing or invalid GitHub repository or workflow token');
  }
  const [owner, repo] = repository.split('/');
  const github = await GitHub.create({
    owner,
    repo,
    defaultBranch: branch,
    token: process.env.GITHUB_TOKEN,
  });
  await updateReleasePullRequest(github);
}

module.exports = {updateReleasePullRequest};
if (require.main === module) {
  main().catch(error => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
