#!/usr/bin/env python3
"""Publish the BCX workspace crates in dependency order.

This script intentionally pauses after publishing dependency crates so
crates.io has time to index them before publishing dependents.

Publish order:
1. bcx-core
2. bcx-policy
3. bcx-wire
4. bcx-crypto
5. bcx-model
6. bcx
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - release host guard.
    print("Python 3.11+ is required because this script uses tomllib.", file=sys.stderr)
    raise


ROOT = Path(__file__).resolve().parents[1]

DEPENDENCY_STEPS = (
    "bcx-core",
    "bcx-policy",
    "bcx-wire",
    "bcx-crypto",
    "bcx-model",
)

FINAL_STEPS = ("bcx",)

ALL_PACKAGES = DEPENDENCY_STEPS + FINAL_STEPS


def run(command: list[str], *, dry_run: bool) -> None:
    print(f"+ {' '.join(command)}", flush=True)
    if dry_run:
        return
    subprocess.run(command, cwd=ROOT, check=True)


def capture(command: list[str]) -> str:
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def workspace_version() -> str:
    with (ROOT / "Cargo.toml").open("rb") as handle:
        manifest = tomllib.load(handle)
    return manifest["workspace"]["package"]["version"]


def cargo_metadata() -> dict:
    metadata = capture(["cargo", "metadata", "--no-deps", "--format-version", "1"])
    return json.loads(metadata)


def workspace_package_versions() -> dict[str, str]:
    metadata = cargo_metadata()
    root = str(ROOT)
    versions = {}
    for entry in metadata["packages"]:
        if entry["manifest_path"].startswith(root):
            versions[entry["name"]] = entry["version"]
    return versions


def require_clean_tree(*, allow_dirty: bool) -> None:
    if allow_dirty:
        return

    status = capture(["git", "status", "--porcelain"])
    if status:
        print("Refusing to publish from a dirty worktree:", file=sys.stderr)
        print(status, file=sys.stderr)
        print("Commit or stash changes, or pass --allow-dirty.", file=sys.stderr)
        sys.exit(1)


def verify_package_set() -> None:
    actual = set(workspace_package_versions())
    expected = set(ALL_PACKAGES)
    if actual != expected:
        missing = sorted(expected - actual)
        extra = sorted(actual - expected)
        raise RuntimeError(
            "workspace package set does not match release script; "
            f"missing={missing}, extra={extra}"
        )


def verify_versions(expected_version: str) -> None:
    for package, actual in sorted(workspace_package_versions().items()):
        if actual != expected_version:
            raise RuntimeError(
                f"{package} is version {actual}, expected {expected_version}"
            )


def check_release_notes(version: str) -> None:
    path = ROOT / "release-notes" / f"RELEASE_NOTES_{version}.md"
    if not path.is_file() or path.stat().st_size == 0:
        raise RuntimeError(f"missing release notes: {path.relative_to(ROOT)}")


def require_no_scratch_pentest() -> None:
    path = ROOT / "PENTEST.md"
    if path.exists():
        raise RuntimeError("PENTEST.md is scratch input and must not be committed")


def check_pentest_report(version: str) -> None:
    tag = f"v{version}"
    path = ROOT / "security" / "pentest" / f"{tag}.md"
    if not path.is_file() or path.stat().st_size == 0:
        raise RuntimeError(f"missing pentest report: {path.relative_to(ROOT)}")

    text = path.read_text(encoding="utf-8")
    required_patterns = (
        r"^Status: PASS$",
        r"^Tester: .+",
        r"^Scope: .+",
        r"^Date: [0-9]{4}-[0-9]{2}-[0-9]{2}$",
        r"^Input-Digest: sha256:[0-9a-fA-F]{64}$",
        r"^Audited-Commit: [0-9a-fA-F]{40}$",
    )
    for pattern in required_patterns:
        if re.search(pattern, text, flags=re.MULTILINE) is None:
            raise RuntimeError(
                f"pentest report {path.relative_to(ROOT)} missing {pattern}"
            )

    audited_commit = re.search(
        r"^Audited-Commit: ([0-9a-fA-F]{40})$", text, flags=re.MULTILINE
    )
    if audited_commit is None:
        raise RuntimeError(f"pentest report {path.relative_to(ROOT)} is malformed")

    audited_hash = audited_commit.group(1)
    if subprocess.run(
        ["git", "merge-base", "--is-ancestor", audited_hash, "HEAD"],
        cwd=ROOT,
        check=False,
    ).returncode != 0:
        raise RuntimeError(f"audited commit {audited_hash} is not an ancestor of HEAD")

    allowed = str(path.relative_to(ROOT))
    changed = [
        line
        for line in capture(["git", "diff", "--name-only", f"{audited_hash}..HEAD"])
        .splitlines()
        if line and line != allowed
    ]
    if changed:
        raise RuntimeError(
            "code changed after audited commit; only "
            f"{allowed} may differ: {changed}"
        )


def check_release_tag(version: str, *, require_tag: bool) -> None:
    tag = f"v{version}"
    tag_ref = f"refs/tags/{tag}"

    if subprocess.run(
        ["git", "rev-parse", "-q", "--verify", tag_ref],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    ).returncode != 0:
        message = f"Release tag {tag!r} was not found."
        if require_tag:
            print(f"Refusing to publish: {message}", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}", file=sys.stderr)
        return

    head = capture(["git", "rev-parse", "HEAD"])
    tagged_commit = capture(["git", "rev-list", "-n", "1", tag_ref])

    if head != tagged_commit:
        message = f"HEAD is not tagged as {tag} (HEAD {head}, {tag} {tagged_commit})."
        if require_tag:
            print(f"Refusing to publish: {message}", file=sys.stderr)
            sys.exit(1)
        print(f"Warning: {message}", file=sys.stderr)
        return

    print(f"Release tag {tag} points at HEAD.")


def check_publish_readiness(version: str, *, require_tag: bool) -> None:
    if not require_tag:
        print("Skipping publish-readiness checks until --require-tag is used.")
        return

    require_no_scratch_pentest()
    check_pentest_report(version)
    print(f"Publish readiness passed for BCX {version}.")


def wait_for_index(package: str, version: str, *, dry_run: bool) -> None:
    url = f"https://crates.io/crates/{package}/{version}"
    print()
    print(f"Published {package} {version}.")
    print(f"Wait until crates.io shows it here: {url}")
    print("Then press Enter to continue with dependent crates.")
    if dry_run:
        print("[dry-run] skipping wait")
        return
    input()


def publish(package: str, args: argparse.Namespace) -> None:
    command = ["cargo", "publish", "-p", package]
    if args.allow_dirty:
        command.append("--allow-dirty")
    if args.no_verify:
        command.append("--no-verify")
    run(command, dry_run=args.dry_run)


def run_preflight(args: argparse.Namespace) -> None:
    if args.skip_checks:
        print("Skipping preflight checks by request.")
        return

    run(["scripts/checks.sh"], dry_run=args.dry_run)


def selected_steps(start_at: str) -> tuple[str, ...]:
    try:
        index = ALL_PACKAGES.index(start_at)
    except ValueError as exc:
        raise RuntimeError(f"unknown package for --start-at: {start_at}") from exc
    return ALL_PACKAGES[index:]


def check_only(version: str) -> None:
    verify_package_set()
    verify_versions(version)
    check_release_notes(version)
    print(f"release script check passed for BCX {version}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Publish BCX workspace crates in crates.io dependency order."
    )
    parser.add_argument(
        "--version",
        default=workspace_version(),
        help="Expected workspace/package version. Defaults to workspace version.",
    )
    parser.add_argument(
        "--start-at",
        default=ALL_PACKAGES[0],
        choices=ALL_PACKAGES,
        help="Resume publishing at a package if an earlier step already succeeded.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Verify release-script metadata without publishing.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print release commands without running them or waiting.",
    )
    parser.add_argument(
        "--allow-dirty",
        action="store_true",
        help="Allow publishing from a dirty worktree and pass --allow-dirty to cargo.",
    )
    parser.add_argument(
        "--skip-checks",
        action="store_true",
        help="Skip local checks before publishing.",
    )
    parser.add_argument(
        "--no-verify",
        action="store_true",
        help="Pass --no-verify to cargo publish. Use only if you understand why.",
    )
    parser.add_argument(
        "--require-tag",
        action="store_true",
        help="Refuse to publish unless HEAD matches the v<version> release tag.",
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="Do not ask for the initial confirmation.",
    )
    args = parser.parse_args()

    if args.check:
        check_only(args.version)
        return 0

    require_clean_tree(allow_dirty=args.allow_dirty or args.dry_run)
    verify_package_set()
    verify_versions(args.version)
    check_release_notes(args.version)
    check_release_tag(args.version, require_tag=args.require_tag)
    check_publish_readiness(args.version, require_tag=args.require_tag)

    steps = selected_steps(args.start_at)

    print(f"Workspace root: {ROOT}")
    print(f"Release version: {args.version}")
    print("Publish sequence:")
    for package in steps:
        print(f"  - {package}")
    print()

    if not args.yes:
        answer = input("Type the release version to start publishing: ").strip()
        if answer != args.version:
            print("Version confirmation did not match; aborting.", file=sys.stderr)
            return 1

    run_preflight(args)

    for package in steps:
        publish(package, args)
        if package in DEPENDENCY_STEPS:
            wait_for_index(package, args.version, dry_run=args.dry_run)

    print()
    print("BCX release publish sequence completed.")
    print("Recommended follow-up:")
    for package in ALL_PACKAGES:
        print(f"  cargo info {package}@{args.version}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
