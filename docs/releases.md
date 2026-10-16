# Releases

## Release automation

The [Release Please workflow](../.github/workflows/release-please.yml) maintains version/changelog PRs for the entire Rust workspace. It runs on every push to `main` and never creates tags or GitHub Releases. It uses the built-in Rust strategy, a root entry in `release-please-config.json`, and `.release-please-manifest.json` as the record of merged version updates.

All five crates use the same version. Their package versions are explicit because the built-in Rust updater does not support `version.workspace = true`. A release PR updates all package versions, version requirements for local path dependencies, the workspace entries in `Cargo.lock`, the release manifest, and `CHANGELOG.md`. Edition and minimum Rust version remain inherited from the workspace. Do not configure each crate as a separate release component.

The initial release manifest is empty: there is no previous version PR. The Rust strategy proposes `0.1.0` for the first release, including the existing feature commits in its changelog. Subsequent versions are computed from commits after the latest commit that updated the version manifest; there is no permanent `release-as` override. A small wrapper calls only Release Please's `createPullRequests()` API, supplies that commit as `lastReleaseSha`, and removes the pending-release label from merged version PRs. This prevents both waiting forever for a disabled tag and repeating old changelog entries. It fails closed if an existing version has no manifest commit within the 500-commit search limit. Do not modify the version manifest independently of a version PR.

## Repository setup

GitHub Actions must be enabled. In **Settings → Actions → General → Workflow permissions**, enable **Allow GitHub Actions to create and approve pull requests**. Organization policies may require an administrator. The workflow declares its own write permissions for contents, issues/labels, and pull requests; the repository-wide default can remain read-only.

The workflow uses the repository's built-in `GITHUB_TOKEN`. No personal access token or crates.io credential is needed. Checkout is pinned to a full commit SHA with Git credential persistence disabled. The workflow installs `release-please@17.3.0` into the runner's temporary directory with lifecycle scripts disabled. No Rust build or application dependencies are installed. `skip-github-release` is also enabled in the configuration as defense in depth.

## Maintainer flow

1. Squash-merge normal PRs into `main` with Conventional Commit titles. Use `feat:`, `fix:`, and `feat!:` / `fix!:` or a `BREAKING CHANGE:` footer as appropriate. The squash commit message, not just the individual branch commits, drives release notes and version selection.
2. A push to `main` creates or updates the release PR when there are releasable changes. Documentation, CI, or maintenance-only commits do not necessarily create a release. The workflow can also be dispatched manually against `main`; dispatches against other branches are skipped.
3. Review the generated changelog, all five package versions, internal dependency versions, and `Cargo.lock`. Run the repository's local checks before merging the release PR; this workflow does not replace testing or coverage checks.
4. Merge the version/changelog PR into `main`. Its versions and changelog become the baseline for the next run. No tag, GitHub Release, or registry upload is created, including on the push caused by this merge.
5. Publish to crates.io manually only after the separate publishing prerequisites have been completed. Tagging or creating GitHub Releases, if ever wanted, is a separate maintainer action.

Before `1.0.0`, fixes increment the patch version; features and breaking changes increment the minor version. Breaking changes must still be called out in commit messages. Once the project reaches `1.0.0`, normal semantic-versioning rules apply.

Default `GITHUB_TOKEN` events do not trigger most other GitHub Actions workflows. This workflow does not emit release/tag events at all. A future publishing workflow must not depend on such events. Use an explicitly designed publishing job or manual dispatch if publishing automation is added later; do not add a broader token merely to run this workflow.

## Local automation checks

The optional local checker validates the configuration against the upstream schema, checks workflow safety, and exercises the real Rust strategy without a GitHub client or any network calls. It simulates the initial release, pre-1.0 patch/feature/breaking releases, stable feature/breaking releases, and a CI-only change. It verifies all five package versions, seven internal dependency requirements, lockfile updates, changelog entries, and the absence of generated releases/tags. Additional lifecycle checks cover merging a version PR without a tag, ignoring older commits in the next changelog, removing the merged PR's pending label, a no-change rerun, and failing closed when the version boundary is unavailable.

Install local verification tools only under the ignored `target/` directory. They are not SDK dependencies; the workflow installs its own copy of the same pinned Release Please package without the verification-only Ajv/YAML tools:

```bash
npm install --prefix target/release-please-check --no-audit --no-fund --ignore-scripts \
  release-please@17.3.0 ajv@8 ajv-formats@3 yaml@2
NODE_PATH="$PWD/target/release-please-check/node_modules" node scripts/check-release-please.cjs
```

Run `actionlint .github/workflows/release-please.yml` separately to validate GitHub Actions syntax and expressions. These checks do not create PRs, GitHub Releases, tags, or registry uploads. The first real workflow run on `main` remains the end-to-end verification of repository permissions and GitHub integration.

## Boundaries

- This workflow creates or updates version/changelog PRs only. It never creates tags or GitHub Releases. It never merges its own PRs or runs `cargo publish`.
- All crates retain `publish = false` until the publishing metadata, SDK license, packaged notices, and registry availability have been reviewed separately.
- The workflow checks out the automation scripts, but requires no Rust toolchain, cloud credentials, or access to Tencent Cloud APIs.
- Concurrent runs are serialized per ref and are not cancelled halfway through release operations.

See the [official Release Please Action documentation](https://github.com/googleapis/release-please-action) and [manifest configuration guide](https://github.com/googleapis/release-please/blob/main/docs/manifest-releaser.md).
