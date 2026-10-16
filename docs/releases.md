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
4. Merge the version/changelog PR into `main`. Its versions and changelog become the baseline for the next run. A separate publishing workflow validates the merged version PR, tests its exact commit, creates `vX.Y.Z`, and publishes the five crates to crates.io.
5. Monitor the **Publish crates** workflow. It publishes core, COS, SES, SSM, and finally the facade; it never creates a GitHub Release. If a job partially completes, manually dispatch **Publish crates** on `main` as described below.

Before `1.0.0`, fixes increment the patch version; features and breaking changes increment the minor version. Breaking changes must still be called out in commit messages. Once the project reaches `1.0.0`, normal semantic-versioning rules apply.

Default `GITHUB_TOKEN` events do not trigger most other GitHub Actions workflows. This workflow does not emit release/tag events at all. The separate publishing workflow listens to pushes to `main` that change the version manifest, not tag or release events. Human merges of version PRs produce such pushes; do not add a broader GitHub token merely to chain workflows.

## Automatic crates.io publishing

The independent [Publish crates workflow](../.github/workflows/publish-crates.yml) runs when a push to `main` changes `.release-please-manifest.json`. The empty initial manifest is skipped. It requires the latest manifest commit to belong to a merged Release Please PR targeting `main`, with the matching generated branch and version title; editing a manifest in an ordinary PR cannot trigger publication. For recovery, manual dispatch is supported only on `main`.

Configure a GitHub Actions repository secret named **`CARGO_REGISTRY_TOKEN`** before merging the first generated version PR. Store the credential directly in GitHub's Secrets UI, never in chat, source, or shell arguments. Its crates.io account must have a verified email, be allowed to create the five package names for their first publication, and own the crates or have the necessary team permissions for updates. Use the narrowest available publish scopes for these exact names. A name being unclaimed is not a reservation; authorization and name availability are enforced by crates.io at publication time. The workflow does not provision credentials, modify owners, or change repository settings.

The prepare job has read-only repository/PR permissions. The publish job checks out the exact version PR commit, not the latest `main` code. It installs Rust 1.85.0, validates all five package versions, seven internal dependency requirements, lockfile, changelog, registry restrictions, licenses and notices, then runs all-feature and no-default-feature tests and strict rustdoc. Registry credentials are present only in the final tag/upload step. Missing registry credentials stop the workflow before creating a tag.

The final step creates a lightweight `vX.Y.Z` tag at that exact commit using the built-in `GITHUB_TOKEN`. An existing lightweight or annotated tag is accepted only if it resolves to the same commit; no tag is moved or replaced. It then publishes these packages sequentially with `cargo publish --locked --registry crates-io`:

1. `tencentcloud-sdk-core`
2. `tencentcloud-sdk-cos`
3. `tencentcloud-sdk-ses`
4. `tencentcloud-sdk-ssm`
5. `tencentcloud-sdk`

Each uploaded archive must contain matching source-commit metadata, LICENSE, and third-party notices. The workflow verifies its SHA-256 against the crates.io sparse index and waits up to 30 index checks (10 seconds apart) before moving to a dependent package. Neither workflow creates a GitHub Release. The publishing workflow is serialized and is never cancelled by a newer version push.

### Recovery

Git tags and registry uploads are not atomic: a tag and some crates can remain if a later upload fails. Do not delete/move that tag or overwrite a registry version. Manually dispatch **Publish crates** on `main` before merging another version PR. It finds the latest version-manifest commit even if unrelated code has subsequently landed on `main`. Existing registry versions are packaged again and skipped only when their checksums match the local archive exactly; a mismatch or yanked version fails closed. Crates.io versions cannot be overwritten. Cargo failures are not blindly retried; a maintainer initiates a new run after inspecting the cause.

Manual dispatch always targets the latest manifest version, not an arbitrary old release. If a newer version PR is merged before a previous partial publication is recovered, recovering the older version requires a separately reviewed process. Do not mark such an incomplete release as successfully published.

## Local automation checks

The optional local checker validates the configuration against the upstream schema, checks workflow safety, and exercises the real Rust strategy without a GitHub client or any network calls. It simulates the initial release, pre-1.0 patch/feature/breaking releases, stable feature/breaking releases, and a CI-only change. It verifies all five package versions, seven internal dependency requirements, lockfile updates, changelog entries, and the absence of generated releases/tags. Additional lifecycle checks cover merging a version PR without a tag, ignoring older commits in the next changelog, removing the merged PR's pending label, a no-change rerun, and failing closed when the version boundary is unavailable.

Install local verification tools only under the ignored `target/` directory. They are not SDK dependencies; the workflow installs its own copy of the same pinned Release Please package without the verification-only Ajv/YAML tools:

```bash
npm install --prefix target/release-please-check --no-audit --no-fund --ignore-scripts \
  release-please@17.3.0 ajv@8 ajv-formats@3 yaml@2
NODE_PATH="$PWD/target/release-please-check/node_modules" node scripts/check-release-please.cjs
```

Run `actionlint .github/workflows/release-please.yml .github/workflows/publish-crates.yml` separately to validate GitHub Actions syntax and expressions. These checks do not create PRs, GitHub Releases, tags, or registry uploads. The first real workflow run on `main` remains the end-to-end verification of repository permissions and GitHub integration.

## Boundaries

- This workflow creates or updates version/changelog PRs only. It never creates tags or GitHub Releases. It never merges its own PRs or runs `cargo publish`.
- All five crates are explicitly restricted to the `crates-io` registry and inherit Apache-2.0 and the repository URL. Each package includes its license, third-party notices, and readme.
- The workflow checks out the automation scripts, but requires no Rust toolchain, cloud credentials, or access to Tencent Cloud APIs.
- Concurrent runs are serialized per ref and are not cancelled halfway through release operations.

See the [official Release Please Action documentation](https://github.com/googleapis/release-please-action) and [manifest configuration guide](https://github.com/googleapis/release-please/blob/main/docs/manifest-releaser.md).
