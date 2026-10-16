# Releases

## Release automation

The [Release Please workflow](../.github/workflows/release-please.yml) manages one release stream for the entire Rust workspace. It uses the built-in Rust strategy, a root entry in `release-please-config.json`, and `.release-please-manifest.json` as the record of released versions.

All five crates use the same version. Their package versions are explicit because the built-in Rust updater does not support `version.workspace = true`. A release PR updates all package versions, version requirements for local path dependencies, the workspace entries in `Cargo.lock`, the release manifest, and `CHANGELOG.md`. Edition and minimum Rust version remain inherited from the workspace. Do not configure each crate as a separate release component.

The initial release manifest is empty: there is no previous release. The Rust strategy proposes `0.1.0` for the first release, including the existing feature commits in its changelog. Subsequent versions are computed from commits since the previous release; there is no permanent `release-as` override.

## Repository setup

GitHub Actions must be enabled. In **Settings → Actions → General → Workflow permissions**, enable **Allow GitHub Actions to create and approve pull requests**. Organization policies may require an administrator. The workflow declares its own write permissions for contents, issues/labels, and pull requests; the repository-wide default can remain read-only.

The workflow uses the repository's built-in `GITHUB_TOKEN`. No personal access token or crates.io credential is needed. The official action is pinned to a full commit SHA; update the pin deliberately after reviewing the upstream release.

## Maintainer flow

1. Squash-merge normal PRs into `main` with Conventional Commit titles. Use `feat:`, `fix:`, and `feat!:` / `fix!:` or a `BREAKING CHANGE:` footer as appropriate. The squash commit message, not just the individual branch commits, drives release notes and version selection.
2. A push to `main` creates or updates the release PR when there are releasable changes. Documentation, CI, or maintenance-only commits do not necessarily create a release. The workflow can also be dispatched manually against `main`; dispatches against other branches are skipped.
3. Review the generated changelog, all five package versions, internal dependency versions, and `Cargo.lock`. Run the repository's local checks before merging the release PR; this workflow does not replace testing or coverage checks.
4. Merge the release PR into `main`. The next workflow run creates the GitHub Release and its `vX.Y.Z` tag at the release commit. No separate tag workflow is needed.
5. Publish to crates.io manually only after the separate publishing prerequisites have been completed. GitHub tagging and release notes do not publish any crate.

Before `1.0.0`, fixes increment the patch version; features and breaking changes increment the minor version. Breaking changes must still be called out in commit messages. Once the project reaches `1.0.0`, normal semantic-versioning rules apply.

Default `GITHUB_TOKEN` events do not trigger most other GitHub Actions workflows. A future publishing workflow must not assume that a release/tag created by this workflow will start it. Use an explicitly designed publishing job or manual dispatch if publishing automation is added later; do not add a broader token merely to run this workflow.

## Local automation checks

The optional local checker validates the configuration against the upstream schema, checks workflow safety, and exercises the real Rust strategy without a GitHub client or any network calls. It simulates the initial release, pre-1.0 patch/feature/breaking releases, stable feature/breaking releases, and a CI-only change. It verifies all five package versions, seven internal dependency requirements, lockfile updates, changelog entries, and generated release tags.

Install verification tools only under the ignored `target/` directory; they are not SDK dependencies and are not used by the workflow:

```bash
npm install --prefix target/release-please-check --no-audit --no-fund --ignore-scripts \
  release-please@17.3.0 ajv@8 ajv-formats@3 yaml@2
NODE_PATH="$PWD/target/release-please-check/node_modules" node scripts/check-release-please.cjs
```

Run `actionlint .github/workflows/release-please.yml` separately to validate GitHub Actions syntax and expressions. These checks do not create PRs, GitHub Releases, tags, or registry uploads. The first real workflow run on `main` remains the end-to-end verification of repository permissions and GitHub integration.

## Boundaries

- This workflow creates release PRs, changelog entries, GitHub Releases, and tags only. It never merges its own PRs or runs `cargo publish`.
- All crates retain `publish = false` until the publishing metadata, SDK license, packaged notices, and registry availability have been reviewed separately.
- The workflow does not require a checkout, Rust toolchain, cloud credentials, or access to Tencent Cloud APIs.
- Concurrent runs are serialized per ref and are not cancelled halfway through release operations.

See the [official Release Please Action documentation](https://github.com/googleapis/release-please-action) and [manifest configuration guide](https://github.com/googleapis/release-please/blob/main/docs/manifest-releaser.md).
