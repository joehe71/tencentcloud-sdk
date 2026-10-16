#!/usr/bin/env python3
"""Publish only a merged Release Please snapshot; use Python's standard library."""

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import time
import tomllib
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ".release-please-manifest.json"
PACKAGES = [
    ("tencentcloud-sdk-core", "crates/core"),
    ("tencentcloud-sdk-cos", "crates/cos"),
    ("tencentcloud-sdk-ses", "crates/ses"),
    ("tencentcloud-sdk-ssm", "crates/ssm"),
    ("tencentcloud-sdk", "."),
]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def valid_version(version):
    require(isinstance(version, str), "Missing release version")
    require(re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?", version), "Invalid SemVer")
    prerelease = version.split("+", 1)[0].partition("-")[2]
    require(all(not part.isdigit() or part == "0" or not part.startswith("0") for part in prerelease.split(".")), "Invalid numeric prerelease identifier")
    return version


def command(args, capture=False):
    environment = {**os.environ, "CARGO_TARGET_DIR": str(ROOT / "target")}
    environment.pop("GH_TOKEN", None)
    if args[:2] != ["cargo", "publish"]:
        environment.pop("CARGO_REGISTRY_TOKEN", None)
    result = subprocess.run(args, cwd=ROOT, check=True, text=True,
                            stdout=subprocess.PIPE if capture else None,
                            env=environment,
                            timeout=600)
    return result.stdout if capture else None


def git(*args):
    return command(["git", *args], capture=True).strip()


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


def fetch(url, token=None, body=None, missing=False):
    headers = {"User-Agent": "tencentcloud-sdk-release", "Cache-Control": "no-cache"}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    if body is not None:
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, headers=headers,
                                     data=json.dumps(body).encode() if body is not None else None)
    try:
        with urllib.request.build_opener(NoRedirect).open(request, timeout=30) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        if missing and error.code == 404:
            return None
        raise ValueError(f"HTTP {error.code} from release service") from None
    except urllib.error.URLError:
        raise ValueError("Unable to contact release service") from None


def github(path, body=None, missing=False):
    repository = os.environ.get("GITHUB_REPOSITORY", "")
    require(re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository), "Invalid GitHub repository")
    token = os.environ.get("GH_TOKEN")
    require(token, "Missing GitHub workflow token")
    response = fetch(f"https://api.github.com/repos/{repository}/{path}", token, body, missing)
    return json.loads(response) if response is not None else None


def prepare():
    versions = json.loads((ROOT / MANIFEST).read_text())
    if versions == {}:
        return {"ready": "false"}
    require(set(versions) == {"."}, "Expected one root release stream")
    version = valid_version(versions["."])
    sha = git("log", "--first-parent", "-1", "--format=%H", "--", MANIFEST)
    require(re.fullmatch(r"[0-9a-f]{40}", sha), "Invalid manifest commit")
    prs = github(f"commits/{sha}/pulls?per_page=100")
    require(any(
        pr.get("merged_at") and pr.get("merge_commit_sha") == sha
        and pr.get("base", {}).get("ref") == "main"
        and re.fullmatch(r"release-please--branches--main(?:--components--tencentcloud-sdk)?", pr.get("head", {}).get("ref", ""))
        and pr.get("title") == f"chore(main): release {version}"
        for pr in prs
    ), "Version manifest must come from a merged Release Please PR targeting main")
    return {"ready": "true", "version": version, "sha": sha}


def validate(version, sha):
    valid_version(version)
    require(re.fullmatch(r"[0-9a-f]{40}", sha), "Invalid release commit")
    require(git("rev-parse", "HEAD") == sha, "Checkout is not the version PR commit")
    require(not git("status", "--porcelain"), "Release checkout must be clean")
    require(json.loads((ROOT / MANIFEST).read_text()) == {".": version}, "Release manifest version mismatch")
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]
    require(set(workspace["members"]) == {path for _, path in PACKAGES if path != "."}, "Unexpected workspace members")
    require(workspace["package"]["license"] == "Apache-2.0", "Unexpected SDK license")
    require(workspace["package"]["repository"] == "https://github.com/joehe71/tencentcloud-sdk", "Unexpected package repository")
    dependency_count = 0
    names = {name for name, _ in PACKAGES}
    for name, path in PACKAGES:
        directory = ROOT / path
        cargo = tomllib.loads((directory / "Cargo.toml").read_text())
        package = cargo["package"]
        require(package["name"] == name and package["version"] == version, f"Version/name mismatch: {name}")
        require(package.get("publish") == ["crates-io"], f"crates.io publication is not enabled: {name}")
        for field in ("license", "repository"):
            require(package.get(field) == {"workspace": True}, f"Expected shared {field}: {name}")
        for file in ("LICENSE", "THIRD_PARTY_NOTICES.md"):
            require((directory / file).read_bytes() == (ROOT / file).read_bytes(), f"Missing or inconsistent {file}: {name}")
            require(file in package.get("include", []), f"Package excludes {file}: {name}")
        require((directory / package["readme"]).is_file(), f"Missing readme: {name}")
        for section in ("dependencies", "dev-dependencies", "build-dependencies"):
            for dependency, spec in cargo.get(section, {}).items():
                if dependency in names:
                    require(spec.get("version") == version and spec.get("path"), f"Internal dependency version mismatch: {name}")
                    dependency_count += 1
    require(dependency_count == 7, "Unexpected internal dependency graph")
    lock = tomllib.loads((ROOT / "Cargo.lock").read_text())
    entries = [package for package in lock["package"] if package["name"] in names]
    require(len(entries) == 5 and all(package["version"] == version for package in entries), "Lockfile version mismatch")
    require(re.search(r"^## \[?" + re.escape(version) + r"(?:\]|\s|$)", (ROOT / "CHANGELOG.md").read_text(), re.MULTILINE), "Missing version changelog entry")
    print(f"Validated five packages and seven dependencies at {version} ({sha})")


def ensure_tag(version, sha):
    tag = f"v{version}"
    existing = github(f"git/ref/tags/{tag}", missing=True)
    if existing is None:
        github("git/refs", {"ref": f"refs/tags/{tag}", "sha": sha})
        print(f"Created {tag} at {sha}")
        return
    target = existing["object"]
    for _ in range(5):
        if target["type"] == "commit":
            require(target["sha"] == sha, "Existing release tag points to another commit; refusing to move it")
            print(f"Reusing {tag} at {sha}")
            return
        require(target["type"] == "tag", "Unexpected Git tag object")
        target = github(f"git/tags/{target['sha']}")["object"]
    raise ValueError("Too many nested annotated tags")


def registry_entry(name, version):
    require(name in {package for package, _ in PACKAGES}, "Unexpected package name")
    valid_version(version)
    data = fetch(f"https://index.crates.io/{name[:2]}/{name[2:4]}/{name}", missing=True)
    if data is None:
        return None
    for line in data.splitlines():
        entry = json.loads(line)
        if entry["vers"] == version:
            require(not entry.get("yanked"), f"Refusing to reuse yanked version: {name}")
            require(re.fullmatch(r"[0-9a-f]{64}", entry["cksum"]), "Invalid registry checksum")
            return entry
    return None


def archive_checksum(name, version, sha):
    archive = ROOT / "target" / "package" / f"{name}-{version}.crate"
    with tarfile.open(archive, "r:gz") as package:
        info = package.extractfile(f"{name}-{version}/.cargo_vcs_info.json")
        require(info is not None, "Archive has no source commit metadata")
        require(json.load(info)["git"]["sha1"] == sha, "Archive source commit mismatch")
        for file in ("LICENSE", "THIRD_PARTY_NOTICES.md"):
            included = package.extractfile(f"{name}-{version}/{file}")
            require(included is not None and included.read() == (ROOT / file).read_bytes(), f"Archive is missing {file}")
    return hashlib.sha256(archive.read_bytes()).hexdigest()


def publish(version, sha):
    validate(version, sha)
    require(os.environ.get("CARGO_REGISTRY_TOKEN"), "Configure the CARGO_REGISTRY_TOKEN GitHub Actions secret before publishing")
    require(os.environ.get("GH_TOKEN"), "Missing GitHub workflow token")
    ensure_tag(version, sha)
    for name, _ in PACKAGES:
        existing = registry_entry(name, version)
        if existing is not None:
            command(["cargo", "package", "--locked", "--registry", "crates-io", "--package", name])
            require(archive_checksum(name, version, sha) == existing["cksum"], f"Existing registry version differs from this source: {name}")
            print(f"Already published, checksum verified: {name} {version}")
            continue
        command(["cargo", "publish", "--locked", "--registry", "crates-io", "--package", name])
        checksum = archive_checksum(name, version, sha)
        # Wait for the sparse index before publishing a dependent crate.
        for attempt in range(30):
            entry = registry_entry(name, version)
            if entry is not None:
                require(entry["cksum"] == checksum, f"Published checksum mismatch: {name}")
                break
            if attempt == 29:
                raise ValueError(f"Registry visibility timeout: {name}; rerun the workflow to resume")
            time.sleep(10)
        print(f"Published and indexed: {name} {version}")


def main():
    require(os.environ.get("GITHUB_REF") == "refs/heads/main", "Publication is restricted to main")
    require(os.environ.get("GITHUB_EVENT_NAME") in ("push", "workflow_dispatch"), "Unexpected workflow event")
    if sys.argv[1:] == ["prepare"]:
        outputs = prepare()
        with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
            for key, value in outputs.items():
                output.write(f"{key}={value}\n")
    elif len(sys.argv) == 4 and sys.argv[1] in ("validate", "publish"):
        {"validate": validate, "publish": publish}[sys.argv[1]](*sys.argv[2:])
    else:
        raise ValueError("Usage: publish-crates.py prepare | validate VERSION SHA | publish VERSION SHA")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, subprocess.SubprocessError, tarfile.TarError) as error:
        print(f"Publication stopped: {error}", file=sys.stderr)
        sys.exit(1)
