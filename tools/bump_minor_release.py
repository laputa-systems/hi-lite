#!/usr/bin/env python3
"""Bump, commit, tag, and publish the next minor crate release."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
MANIFEST = ROOT / "Cargo.toml"
VERSION_PATTERN = re.compile(
    r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
    r"(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$"
)


def run(*command: str, capture_output: bool = False) -> str:
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        text=True,
        capture_output=capture_output,
    )
    if result.returncode:
        if capture_output:
            sys.stderr.write(result.stdout)
            sys.stderr.write(result.stderr)
        raise subprocess.CalledProcessError(result.returncode, command)
    return result.stdout if capture_output else ""


def version_tuple(version: str) -> tuple[int, int, int]:
    match = VERSION_PATTERN.fullmatch(version)
    if match is None:
        raise ValueError(f"cannot bump non-semver package version {version!r}")
    return tuple(map(int, match.groups()))


def package_metadata() -> tuple[str, str]:
    metadata = json.loads(run("cargo", "metadata", "--no-deps", "--format-version", "1", capture_output=True))
    packages = [
        package
        for package in metadata["packages"]
        if Path(package["manifest_path"]).resolve() == MANIFEST.resolve()
    ]
    if len(packages) != 1:
        raise RuntimeError("expected one package in the root Cargo.toml")
    package = packages[0]
    return package["name"], package["version"]


def published_version(package_name: str) -> str:
    output = run("cargo", "search", package_name, "--limit", "1", capture_output=True)
    match = re.search(
        rf"(?m)^{re.escape(package_name)}\s+=\s+\"([^\"]+)\"",
        output,
    )
    if match is None:
        raise RuntimeError(f"could not find {package_name!r} on the default Cargo registry")
    version = match.group(1)
    version_tuple(version)
    return version


def update_manifest_version(old_version: str, new_version: str) -> None:
    text = MANIFEST.read_text()
    lines = text.splitlines(keepends=True)
    section = None
    updated = False
    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            section = stripped
        elif section == "[package]" and re.match(r"^version\s*=", stripped):
            if stripped != f'version = "{old_version}"':
                raise RuntimeError("root Cargo.toml version changed while preparing release")
            newline = "\n" if line.endswith("\n") else ""
            lines[index] = f'version = "{new_version}"{newline}'
            updated = True
            break
    if not updated:
        raise RuntimeError("could not find the root package version in Cargo.toml")
    MANIFEST.write_text("".join(lines))


def ensure_clean_tree() -> None:
    if run("git", "status", "--porcelain=v1", capture_output=True):
        raise RuntimeError("make bump requires a clean working tree")


def release_subject(package_name: str, version: str) -> str:
    return f"release: {package_name} v{version}"


def main() -> None:
    # This authenticated, read-only registry call runs before any release edits.
    run("cargo", "owner", "--list")

    package_name, current_version = package_metadata()
    latest_version = published_version(package_name)
    latest = version_tuple(latest_version)
    current = version_tuple(current_version)
    subject = run("git", "log", "-1", "--format=%s", capture_output=True).strip()
    release_tag = f"v{current_version}"
    tags_at_head = set(run("git", "tag", "--points-at", "HEAD", capture_output=True).splitlines())

    if subject == release_subject(package_name, current_version):
        ensure_clean_tree()
        if release_tag not in tags_at_head:
            run("git", "tag", "-a", release_tag, "-m", f"laputa-hi-lite {current_version}")
        if latest >= current:
            print(f"{package_name} {current_version} is already published; nothing to do")
            return
        print(f"Retrying publish of {package_name} {current_version}")
        run("cargo", "publish")
        return

    if current > latest:
        raise RuntimeError(
            f"local version {current_version} is newer than published {latest_version}, "
            "but HEAD is not the matching release commit"
        )

    ensure_clean_tree()
    base = max(current, latest)
    next_version = f"{base[0]}.{base[1] + 1}.0"
    next_tag = f"v{next_version}"
    existing_tags = set(run("git", "tag", capture_output=True).splitlines())
    if next_tag in existing_tags:
        raise RuntimeError(f"tag {next_tag} already exists away from the release commit")

    original_manifest = MANIFEST.read_text()
    lockfile = ROOT / "Cargo.lock"
    original_lockfile = lockfile.read_text()
    committed = False
    try:
        update_manifest_version(current_version, next_version)
        run("cargo", "check", "--quiet")
        run("cargo", "publish", "--dry-run", "--allow-dirty")
        run("git", "add", "--", "Cargo.toml", "Cargo.lock")
        run("git", "commit", "-m", release_subject(package_name, next_version))
        committed = True
    except BaseException:
        if not committed:
            head_subject = run("git", "log", "-1", "--format=%s", capture_output=True).strip()
            if head_subject != release_subject(package_name, next_version):
                run("git", "reset", "--", "Cargo.toml", "Cargo.lock")
                MANIFEST.write_text(original_manifest)
                lockfile.write_text(original_lockfile)
        raise
    run("git", "tag", "-a", next_tag, "-m", f"laputa-hi-lite {next_version}")
    run("cargo", "publish")


if __name__ == "__main__":
    try:
        main()
    except (OSError, RuntimeError, ValueError, subprocess.CalledProcessError) as error:
        print(f"make bump: {error}", file=sys.stderr)
        sys.exit(1)
