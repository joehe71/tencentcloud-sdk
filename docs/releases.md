# Releases

## Release automation

The [Release Please workflow](../.github/workflows/release-please.yml) maintains version/changelog PRs for the entire Rust workspace. It runs on every push to `main` and never creates tags or GitHub Releases. It uses the built-in Rust strategy, a root entry in `release-please-config.json`, and `.release-please-manifest.json` as the record of merged version updates.

All five crates use the same version. Their package versions are explicit because the built-in Rust updater does not support `version.workspace = true`. A release PR updates all package versions, version requirements for local path dependencies, the workspace entries in `Cargo.lock`, the release manifest, and `CHANGELOG.md`. Edition and minimum Rust version remain inherited from the workspace. Do not configure each crate as a separate release component.

The initial release manifest is empty: there is no previous version PR. The Rust strategy proposes `0.1.0` for the first release, including the existing feature commits in its changelog. Subsequent versions are computed from commits after the latest commit that updated the version manifest; there is no permanent `release-as` override. A small wrapper calls only Release Please's `createPullRequests()` API, supplies that commit as `lastReleaseSha`, and removes the pending-release label from merged version PRs. This prevents both waiting forever for a disabled tag and repeating old changelog entries. It fails closed if an existing version has no manifest commit within the 500-commit search limit. Do not modify the version manifest independently of a version PR.

## Repository setup

GitHub Actions must be enabled. In **Settings → Actions → General → Workflow permissions**, enable **Allow GitHub Actions to create and approve pull requests**. Organization policies may require an administrator. The workflow declares its own write permissions for contents, issues/labels, and pull requests; the repository-wide default can remain read-only.

The workflow uses the repository's built-in `GITHUB_TOKEN`. No personal access token or crates.io credential is needed. Checkout uses a major-version tag with Git credential persistence disabled. The workflow installs `release-please@17.3.0` into the runner's temporary directory with lifecycle scripts disabled. No Rust build or application dependencies are installed. `skip-github-release` is also enabled in the configuration as defense in depth.

## Maintainer flow

1. Squash-merge normal PRs into `main` with Conventional Commit titles. Use `feat:`, `fix:`, and `feat!:` / `fix!:` or a `BREAKING CHANGE:` footer as appropriate. The squash commit message, not just the individual branch commits, drives release notes and version selection.
2. A push to `main` creates or updates the release PR when there are releasable changes. Documentation, CI, or maintenance-only commits do not necessarily create a release. The workflow can also be dispatched manually against `main`; dispatches against other branches are skipped.
3. Review the generated changelog, all five package versions, internal dependency versions, and `Cargo.lock`. Run the repository's local checks before merging the release PR; this workflow does not replace testing or coverage checks.
4. Merge the version/changelog PR into `main`. Its versions and changelog become the baseline for the next run. A separate publishing workflow validates the merged version PR, tests its exact commit, creates `vX.Y.Z` and its GitHub Release, and publishes the five crates to crates.io.
5. Monitor the **Publish crates** workflow. It creates the source tag and GitHub Release before publishing core, COS, SES, SSM, and finally the facade. A visible GitHub Release alone does not mean all crates are available yet. If a job partially completes, manually dispatch **Publish crates** on `main` as described below.

Before `1.0.0`, fixes increment the patch version; features and breaking changes increment the minor version. Breaking changes must still be called out in commit messages. Once the project reaches `1.0.0`, normal semantic-versioning rules apply.

Default `GITHUB_TOKEN` events do not trigger most other GitHub Actions workflows. Release Please itself creates no tags or Releases. The separate publishing workflow listens to pushes to `main` that change the version manifest, then creates the tag and GitHub Release directly in the same job; it does not rely on a second workflow receiving those events. A manually pushed tag alone does not trigger this publishing path. Human merges of version PRs produce the required manifest push; do not add a broader GitHub token merely to chain workflows.

## Automatic crates.io publishing

The independent [Publish crates workflow](../.github/workflows/publish-crates.yml) runs when a push to `main` changes `.release-please-manifest.json`. The empty initial manifest is skipped. It requires the latest manifest commit to belong to a merged Release Please PR targeting `main`, with the matching generated branch and version title; editing a manifest in an ordinary PR cannot trigger publication. For recovery, manual dispatch is supported only on `main`.

Configure a GitHub Actions repository secret named **`CARGO_REGISTRY_TOKEN`** before merging the first generated version PR. Store the credential directly in GitHub's Secrets UI, never in chat, source, or shell arguments. Its crates.io account must have a verified email, be allowed to create the five package names for their first publication, and own the crates or have the necessary team permissions for updates. Use the narrowest available publish scopes for these exact names. A name being unclaimed is not a reservation; authorization and name availability are enforced by crates.io at publication time. The workflow does not provision credentials, modify owners, or change repository settings.

The prepare job has read-only repository/PR permissions. The publish job checks out the exact version PR commit, not the latest `main` code. It installs Rust 1.85.0, validates all five package versions, seven internal dependency requirements, lockfile, nonempty version changelog notes, registry restrictions, licenses and README attribution, then runs all-feature and no-default-feature tests and strict rustdoc. Registry credentials are present only in the final tag/Release/upload step. Missing registry credentials stop the workflow before creating a tag or GitHub Release.

The final step creates a lightweight `vX.Y.Z` tag at that exact commit using the built-in `GITHUB_TOKEN`. An existing lightweight or annotated tag is accepted only if it resolves to the same commit; no tag is moved or replaced. Immediately after verifying the tag, the workflow creates a published GitHub Release named `vX.Y.Z`, using only that version's section from the checked-in `CHANGELOG.md` as its notes. SemVer prereleases are marked as prereleases; build metadata alone does not make a version a prerelease. An existing published Release for the verified tag is reused without changing its title, notes, or assets; a draft or conflicting prerelease status stops the workflow for inspection. The existing `contents: write` permission and built-in token suffice; no additional Action, PAT, or SDK dependency is needed.

The workflow then publishes these packages sequentially with `cargo publish --locked --registry crates-io`:

1. `tencentcloud-sdk-foundation` (Rust library name `tencentcloud_sdk_core`)
2. `tencentcloud-sdk-cos`
3. `tencentcloud-sdk-ses`
4. `tencentcloud-sdk-ssm`
5. `tencentcloud-sdk`

Each uploaded archive must contain matching source-commit metadata, LICENSE, and README with third-party attribution. The workflow verifies its SHA-256 against the crates.io sparse index and waits up to 30 index checks (10 seconds apart) before moving to a dependent package. The GitHub Release records the source tag, not the completion of crates.io uploads; inspect the workflow result to confirm publication of all five packages. The publishing workflow is serialized and is never cancelled by a newer version push.

### Recovery

Git tags, GitHub Releases, and registry uploads are not atomic: a tag can remain if Release creation fails, and both the tag and Release can remain if a later upload fails. Do not delete/move that tag, recreate a published Release, or overwrite a registry version. Manually dispatch **Publish crates** on `main` before merging another version PR. It finds the latest version-manifest commit even if unrelated code has subsequently landed on `main`. The workflow first verifies the existing tag and reuses its published GitHub Release, or creates the missing Release from the same version's notes. Existing registry versions are packaged again with stable Cargo 1.85-compatible `cargo package --locked --package NAME` and skipped only when their checksums match the local archive exactly; a mismatch or yanked version fails closed. The package command deliberately omits `--registry`, which requires nightly in Cargo 1.85. Registry selection remains explicit for `cargo publish`, all manifests restrict publication to `crates-io`, and checksum lookups use the crates.io index. Crates.io versions cannot be overwritten. Cargo failures are not blindly retried; a maintainer initiates a new run after inspecting the cause.

Manual dispatch always targets the latest manifest version, not an arbitrary old release. If a newer version PR is merged before a previous partial publication is recovered, recovering the older version requires a separately reviewed process. Do not mark such an incomplete release as successfully published.

### Initial 0.1.0 package-name collision

The source tag and GitHub Release `v0.1.0` exist, but that snapshot refers to the unrelated, already-published `tencentcloud-sdk-core` package. Its registry checksum does not match this repository. Do not overwrite or reuse that registry version, move the tag, or treat the GitHub Release as successful crates.io publication.

The replacement package is `tencentcloud-sdk-foundation`; its `[lib]` name remains `tencentcloud_sdk_core`, preserving Rust imports. The other four package names are unchanged. After merging the name/packaging fix, merge the next generated version PR to publish a new validated snapshot. Do not rerun the original failed job expecting it to use new scripts or names: publication intentionally checks out the exact version-PR commit, so `v0.1.0` remains the old snapshot. Package-name availability is checked before implementation but is not a reservation; crates.io enforces availability and ownership at upload time. Ensure `CARGO_REGISTRY_TOKEN` permits creating/publishing `tencentcloud-sdk-foundation`; a token restricted to the old core name is insufficient.

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

- Release Please creates or updates version/changelog PRs only. It never creates tags or GitHub Releases, merges its own PRs, or runs `cargo publish`. The independent publishing workflow owns tag creation, GitHub Releases, and crates.io uploads.
- All five crates are explicitly restricted to the `crates-io` registry and inherit Apache-2.0 and the repository URL. Each package includes its license and readme; third-party attribution is retained in the readme rather than a separate notices file.
- Release Please checks out only the automation scripts and requires no Rust toolchain. The publishing job installs Rust for package validation and upload; neither workflow needs Tencent Cloud credentials or calls Tencent Cloud APIs.
- Concurrent runs are serialized per ref and are not cancelled halfway through release operations.

See the [official Release Please Action documentation](https://github.com/googleapis/release-please-action) and [manifest configuration guide](https://github.com/googleapis/release-please/blob/main/docs/manifest-releaser.md).
