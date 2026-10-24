"""Offline release validation tests; no credentials, registry, or GitHub calls."""

import hashlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import tarfile
import tempfile
import tomllib
import unittest
from unittest.mock import patch

REPOSITORY = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("publisher", REPOSITORY / "scripts/publish-crates.py")
publisher = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(publisher)
SHA = "a" * 40


class NoticeValidationTests(unittest.TestCase):
    def setUp(self):
        target = REPOSITORY / "target"
        target.mkdir(exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix="release-validation-", dir=target)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        for _, relative in publisher.PACKAGES:
            directory = self.root / relative
            directory.mkdir(parents=True, exist_ok=True)
            for filename in ("Cargo.toml", "LICENSE", "README.md", "NOTICE"):
                shutil.copyfile(REPOSITORY / relative / filename, directory / filename)
        for filename in (publisher.MANIFEST, "Cargo.lock", "CHANGELOG.md"):
            shutil.copyfile(REPOSITORY / filename, self.root / filename)
        self.version = json.loads((self.root / publisher.MANIFEST).read_text())["."]
        self.addCleanup(patch.stopall)
        patch.object(publisher, "ROOT", self.root).start()
        patch.object(publisher, "git", side_effect=lambda *args: SHA if args == ("rev-parse", "HEAD") else "").start()
        patch.object(publisher, "command", side_effect=AssertionError("unexpected process call")).start()
        patch.object(publisher, "fetch", side_effect=AssertionError("unexpected network call")).start()

    def validate(self):
        publisher.validate(self.version, SHA)

    def test_slim_readmes_with_notice_pass_for_all_five_packages(self):
        for _, relative in publisher.PACKAGES:
            (self.root / relative / "README.md").write_text("# SDK\n\nSee NOTICE.\n")
        self.validate()

    def test_missing_notice_is_rejected_for_every_package(self):
        for _, relative in publisher.PACKAGES:
            with self.subTest(package=relative):
                path = self.root / relative / "NOTICE"
                original = path.read_bytes()
                path.unlink()
                with self.assertRaisesRegex(ValueError, "Missing .*NOTICE"):
                    self.validate()
                path.write_bytes(original)

    def test_notice_must_be_included_for_every_package(self):
        for _, relative in publisher.PACKAGES:
            with self.subTest(package=relative):
                path = self.root / relative / "Cargo.toml"
                original = path.read_text()
                path.write_text(original.replace(', "NOTICE"', ''))
                with self.assertRaisesRegex(ValueError, "Package excludes NOTICE"):
                    self.validate()
                path.write_text(original)

    def test_incomplete_attribution_is_rejected_even_if_readme_contains_it(self):
        path = self.root / "crates/ses/NOTICE"
        (path.parent / "README.md").write_bytes(path.read_bytes())
        path.write_text("Apache-2.0\n")
        with self.assertRaisesRegex(ValueError, "Missing third-party attribution in NOTICE"):
            self.validate()

    def test_different_package_notice_is_rejected(self):
        path = self.root / "crates/cos/NOTICE"
        path.write_text(path.read_text() + "Unexpected divergence.\n")
        with self.assertRaisesRegex(ValueError, "Inconsistent NOTICE"):
            self.validate()

    def test_every_attribution_marker_is_required(self):
        content = (self.root / "NOTICE").read_text()
        for marker in (
            "https://github.com/TencentCloud/tencentcloud-sdk-java",
            "3.2.40",
            "Copyright (c) 2017-2025 Tencent. All Rights Reserved.",
            "Apache-2.0",
            "Annotated protocol field names/types and operation identifiers were transformed",
        ):
            with self.subTest(marker=marker):
                with self.assertRaisesRegex(ValueError, "in NOTICE"):
                    publisher.validate_attribution(content.replace(marker, ""), "test-package")

    def archive(self, name, relative, overrides=None, omitted=(), sha=SHA):
        directory = self.root / "target/package"
        directory.mkdir(parents=True, exist_ok=True)
        archive = directory / f"{name}-{self.version}.crate"
        contents = {".cargo_vcs_info.json": json.dumps({"git": {"sha1": sha}}).encode()}
        for filename in ("LICENSE", "README.md", "NOTICE"):
            contents[filename] = (self.root / relative / filename).read_bytes()
        contents.update(overrides or {})
        with tarfile.open(archive, "w:gz") as package:
            for filename, content in contents.items():
                if filename in omitted:
                    continue
                info = tarfile.TarInfo(f"{name}-{self.version}/{filename}")
                info.size = len(content)
                package.addfile(info, io.BytesIO(content))
        return archive

    def test_matching_notice_archives_pass_for_all_five_packages(self):
        for name, relative in publisher.PACKAGES:
            with self.subTest(package=name):
                archive = self.archive(name, relative)
                self.assertEqual(
                    publisher.archive_checksum(name, self.version, SHA),
                    hashlib.sha256(archive.read_bytes()).hexdigest(),
                )

    def test_missing_or_modified_archive_files_are_rejected(self):
        for name, relative in publisher.PACKAGES:
            for filename in ("LICENSE", "README.md", "NOTICE"):
                for missing in (True, False):
                    with self.subTest(package=name, filename=filename, missing=missing):
                        self.archive(name, relative,
                                     overrides={} if missing else {filename: b"modified"},
                                     omitted=(filename,) if missing else ())
                        with self.assertRaisesRegex(ValueError, f"Archive has missing or inconsistent {filename}"):
                            publisher.archive_checksum(name, self.version, SHA)

    def test_wrong_archive_commit_still_fails_closed(self):
        name, relative = publisher.PACKAGES[0]
        self.archive(name, relative, sha="b" * 40)
        with self.assertRaisesRegex(ValueError, "Archive source commit mismatch"):
            publisher.archive_checksum(name, self.version, SHA)

    def test_versions_are_still_checked(self):
        with self.assertRaisesRegex(ValueError, "Release manifest version mismatch"):
            publisher.validate("9.9.9", SHA)

    def test_dirty_checkout_still_fails_closed(self):
        with patch.object(publisher, "git", side_effect=[SHA, " M README.md"]):
            with self.assertRaisesRegex(ValueError, "Release checkout must be clean"):
                self.validate()

    def test_wrong_checkout_still_fails_closed(self):
        with patch.object(publisher, "git", return_value="b" * 40):
            with self.assertRaisesRegex(ValueError, "Checkout is not the version PR commit"):
                self.validate()

    def test_repository_metadata_passes_without_publication(self):
        with patch.object(publisher, "ROOT", REPOSITORY):
            manifest = tomllib.loads((REPOSITORY / "Cargo.toml").read_text())
            publisher.validate(manifest["package"]["version"], SHA)


if __name__ == "__main__":
    unittest.main()
