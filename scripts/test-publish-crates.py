#!/usr/bin/env python3
"""Offline publishing regressions: no cloud, GitHub writes, or registry uploads."""

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

sys.dont_write_bytecode = True

SPEC = importlib.util.spec_from_file_location("publish_crates", Path(__file__).with_name("publish-crates.py"))
release = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(release)
SOURCE = release.ROOT
SHA = "a" * 40
CHECKSUM = "b" * 64
ENTRY = {"vers": "0.1.0", "cksum": CHECKSUM, "yanked": False}


class PublishingTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(dir=SOURCE / "target")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for _, path in release.PACKAGES:
            destination = self.root / path
            destination.mkdir(parents=True, exist_ok=True)
            for file in ("Cargo.toml", "README.md", "LICENSE", "THIRD_PARTY_NOTICES.md"):
                shutil.copyfile(SOURCE / path / file, destination / file)
        shutil.copyfile(SOURCE / "Cargo.lock", self.root / "Cargo.lock")
        (self.root / release.MANIFEST).write_text('{".": "0.1.0"}')
        (self.root / "CHANGELOG.md").write_text("# Changelog\n\n## [0.1.0] - initial SDK\n")
        for context in (
            patch.object(release, "ROOT", self.root),
            patch.object(release, "fetch", side_effect=AssertionError("Network is forbidden in offline tests")),
            patch.object(release, "git", side_effect=lambda *args: "" if args[0] == "status" else SHA),
            patch.dict(os.environ, {"GITHUB_REF": "refs/heads/main", "GITHUB_EVENT_NAME": "push", "GH_TOKEN": "fake-workflow-token", "CARGO_REGISTRY_TOKEN": "fake-registry-token"}),
        ):
            context.start()
            self.addCleanup(context.stop)

    def merged_pr(self):
        return {"merged_at": "2026-10-16", "merge_commit_sha": SHA,
                "base": {"ref": "main"},
                "head": {"ref": "release-please--branches--main--components--tencentcloud-sdk"},
                "title": "chore(main): release 0.1.0"}

    def test_semver_and_injection_rejection(self):
        for version in ("0.1.0", "1.2.3", "1.0.0-rc.1", "1.0.0+build.2"):
            self.assertEqual(release.valid_version(version), version)
        for version in (None, "", "01.0.0", "1.0.0-01", "1.0.0\nsha=evil", "$(command)", "1/2/3"):
            with self.subTest(version=version), self.assertRaises(ValueError):
                release.valid_version(version)

    def test_initial_manifest_is_skipped_without_api_calls(self):
        (self.root / release.MANIFEST).write_text("{}")
        self.assertEqual(release.prepare(), {"ready": "false"})

    def test_prepare_pins_merged_version_commit(self):
        with patch.object(release, "github", return_value=[self.merged_pr()]) as api:
            self.assertEqual(release.prepare(), {"ready": "true", "version": "0.1.0", "sha": SHA})
        api.assert_called_once_with(f"commits/{SHA}/pulls?per_page=100")
        release.git.assert_called_once_with("log", "--first-parent", "-1", "--format=%H", "--", release.MANIFEST)

    def test_prepare_rejects_ordinary_or_unmerged_prs(self):
        for field, value in (("merged_at", None), ("merge_commit_sha", "c" * 40),
                             ("base", {"ref": "other"}), ("head", {"ref": "ordinary-feature"}),
                             ("title", "chore(main): release 9.9.9")):
            pr = {**self.merged_pr(), field: value}
            with self.subTest(field=field), patch.object(release, "github", return_value=[pr]):
                with self.assertRaisesRegex(ValueError, "merged Release Please PR"):
                    release.prepare()

    def test_validate_all_package_metadata(self):
        release.validate("0.1.0", SHA)

    def test_metadata_version_and_publish_guards(self):
        file = self.root / "crates/core/Cargo.toml"
        original = file.read_text()
        for modified in (original.replace('version = "0.1.0"', 'version = "0.2.0"', 1),
                         original.replace('publish = ["crates-io"]', 'publish = false')):
            file.write_text(modified)
            with self.assertRaises(ValueError):
                release.validate("0.1.0", SHA)
        file.write_text(original)
        with self.assertRaisesRegex(ValueError, "Checkout"):
            release.validate("0.1.0", "c" * 40)

    def test_validate_rejects_missing_notice_and_changelog(self):
        (self.root / "crates/core/THIRD_PARTY_NOTICES.md").write_text("wrong notice")
        with self.assertRaisesRegex(ValueError, "THIRD_PARTY_NOTICES"):
            release.validate("0.1.0", SHA)
        shutil.copyfile(self.root / "THIRD_PARTY_NOTICES.md", self.root / "crates/core/THIRD_PARTY_NOTICES.md")
        (self.root / "CHANGELOG.md").write_text("# Changelog")
        with self.assertRaisesRegex(ValueError, "changelog"):
            release.validate("0.1.0", SHA)

    def test_tag_creation_reuse_and_conflict(self):
        with patch.object(release, "github", return_value=None) as api:
            release.ensure_tag("0.1.0", SHA)
            self.assertEqual(api.call_args_list[-1].args, ("git/refs", {"ref": "refs/tags/v0.1.0", "sha": SHA}))
        for target, fail in ((SHA, False), ("c" * 40, True)):
            with patch.object(release, "github", return_value={"object": {"type": "commit", "sha": target}}) as api:
                if fail:
                    with self.assertRaisesRegex(ValueError, "refusing to move"):
                        release.ensure_tag("0.1.0", SHA)
                else:
                    release.ensure_tag("0.1.0", SHA)
                self.assertEqual(api.call_count, 1)

    def test_annotated_tag_is_peeled(self):
        with patch.object(release, "github", side_effect=[
            {"object": {"type": "tag", "sha": "c" * 40}},
            {"object": {"type": "commit", "sha": SHA}},
        ]) as api:
            release.ensure_tag("0.1.0", SHA)
            self.assertEqual(api.call_count, 2)

    def test_sparse_index_missing_and_yanked_versions(self):
        with patch.object(release, "fetch", return_value=None):
            self.assertIsNone(release.registry_entry("tencentcloud-sdk-core", "0.1.0"))
        with patch.object(release, "fetch", return_value=json.dumps(ENTRY).encode()):
            self.assertEqual(release.registry_entry("tencentcloud-sdk-core", "0.1.0"), ENTRY)
        with patch.object(release, "fetch", return_value=json.dumps({**ENTRY, "yanked": True}).encode()):
            with self.assertRaisesRegex(ValueError, "yanked"):
                release.registry_entry("tencentcloud-sdk-core", "0.1.0")

    def test_publish_dependency_order_and_index_visibility(self):
        with patch.object(release, "ensure_tag") as tag, \
                patch.object(release, "registry_entry", side_effect=[None, None, ENTRY] * 5), \
                patch.object(release, "archive_checksum", return_value=CHECKSUM), \
                patch.object(release, "command") as cargo, patch.object(release.time, "sleep") as sleep:
            release.publish("0.1.0", SHA)
            tag.assert_called_once_with("0.1.0", SHA)
            self.assertEqual([call.args[0] for call in cargo.call_args_list], [
                ["cargo", "publish", "--locked", "--registry", "crates-io", "--package", name]
                for name, _ in release.PACKAGES])
            self.assertEqual(sleep.call_count, 5)

    def test_rerun_verifies_checksums_instead_of_uploading_twice(self):
        with patch.object(release, "ensure_tag"), patch.object(release, "registry_entry", return_value=ENTRY), \
                patch.object(release, "archive_checksum", return_value=CHECKSUM), patch.object(release, "command") as cargo:
            release.publish("0.1.0", SHA)
            self.assertEqual(len(cargo.call_args_list), 5)
            self.assertTrue(all(call.args[0][1] == "package" for call in cargo.call_args_list))
        with patch.object(release, "ensure_tag"), patch.object(release, "registry_entry", return_value=ENTRY), \
                patch.object(release, "archive_checksum", return_value="c" * 64), patch.object(release, "command") as cargo:
            with self.assertRaisesRegex(ValueError, "differs from this source"):
                release.publish("0.1.0", SHA)
            self.assertEqual(cargo.call_count, 1)

    def test_missing_credentials_prevent_tag_and_upload(self):
        with patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": ""}), patch.object(release, "ensure_tag") as tag:
            with self.assertRaisesRegex(ValueError, "CARGO_REGISTRY_TOKEN"):
                release.publish("0.1.0", SHA)
            tag.assert_not_called()

    def test_upload_failure_does_not_blindly_retry(self):
        failure = subprocess.CalledProcessError(101, ["cargo", "publish"])
        with patch.object(release, "ensure_tag"), patch.object(release, "registry_entry", return_value=None), \
                patch.object(release, "command", side_effect=failure) as cargo:
            with self.assertRaises(subprocess.CalledProcessError):
                release.publish("0.1.0", SHA)
            self.assertEqual(cargo.call_count, 1)

    def test_index_timeout_is_bounded(self):
        with patch.object(release, "ensure_tag"), patch.object(release, "registry_entry", return_value=None), \
                patch.object(release, "archive_checksum", return_value=CHECKSUM), \
                patch.object(release, "command") as cargo, patch.object(release.time, "sleep") as sleep:
            with self.assertRaisesRegex(ValueError, "visibility timeout"):
                release.publish("0.1.0", SHA)
            self.assertEqual(cargo.call_count, 1)
            self.assertEqual(sleep.call_count, 29)

    def test_archive_license_notice_commit_and_checksum(self):
        name, version = "tencentcloud-sdk-core", "0.1.0"
        directory = self.root / "target/package"
        directory.mkdir(parents=True)
        archive = directory / f"{name}-{version}.crate"
        with tarfile.open(archive, "w:gz") as package:
            for file, content in {
                ".cargo_vcs_info.json": json.dumps({"git": {"sha1": SHA}}).encode(),
                "LICENSE": (self.root / "LICENSE").read_bytes(),
                "THIRD_PARTY_NOTICES.md": (self.root / "THIRD_PARTY_NOTICES.md").read_bytes(),
            }.items():
                info = tarfile.TarInfo(f"{name}-{version}/{file}")
                info.size = len(content)
                package.addfile(info, io.BytesIO(content))
        self.assertEqual(release.archive_checksum(name, version, SHA), hashlib.sha256(archive.read_bytes()).hexdigest())
        with self.assertRaisesRegex(ValueError, "source commit"):
            release.archive_checksum(name, version, "c" * 40)

    def test_child_commands_receive_only_required_credentials(self):
        with patch.object(release.subprocess, "run") as run:
            for args in (["git", "status"], ["cargo", "package"], ["cargo", "publish"]):
                release.command(args)
                environment = run.call_args.kwargs["env"]
                self.assertNotIn("GH_TOKEN", environment)
                if args == ["cargo", "publish"]:
                    self.assertEqual(environment["CARGO_REGISTRY_TOKEN"], "fake-registry-token")
                else:
                    self.assertNotIn("CARGO_REGISTRY_TOKEN", environment)

    def test_main_rejects_other_refs_and_events(self):
        for environment in ({"GITHUB_REF": "refs/heads/feature"}, {"GITHUB_EVENT_NAME": "pull_request"}):
            with patch.dict(os.environ, environment), self.assertRaises(ValueError):
                release.main()

    def test_cli_prepare_outputs_and_no_redirects(self):
        output = self.root / "outputs"
        with patch.dict(os.environ, {"GITHUB_OUTPUT": str(output)}), \
                patch.object(sys, "argv", ["publish-crates.py", "prepare"]), \
                patch.object(release, "github", return_value=[self.merged_pr()]):
            release.main()
        self.assertEqual(output.read_text(), f"ready=true\nversion=0.1.0\nsha={SHA}\n")
        self.assertIsNone(release.NoRedirect().redirect_request(None, None, 302, "redirect", {}, "https://example.invalid"))


if __name__ == "__main__":
    unittest.main(verbosity=2)
