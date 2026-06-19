#!/usr/bin/env python3
"""Finalize a BCX release after root PENTEST.md is ready."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def run(command: list[str]) -> None:
    print(f"+ {' '.join(command)}", flush=True)
    subprocess.run(command, cwd=ROOT, check=True)


def capture(command: list[str]) -> str:
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def workspace_version() -> str:
    cargo_toml = ROOT / "Cargo.toml"
    for line in cargo_toml.read_text(encoding="utf-8").splitlines():
        if line.startswith("version = "):
            return line.split('"', 2)[1]
    raise RuntimeError("could not determine workspace version")


def require_clean_release_tree(report: Path) -> None:
    status = capture(["git", "status", "--porcelain"])
    report_path = str(report.relative_to(ROOT))
    tracked_or_unignored = [
        line
        for line in status.splitlines()
        if not line.endswith(" PENTEST.md")
        and not line.endswith(f" {report_path}")
        and "PENTEST.md" not in line
    ]
    if tracked_or_unignored:
        print("Refusing to finalize from a dirty tracked worktree:", file=sys.stderr)
        print("\n".join(tracked_or_unignored), file=sys.stderr)
        sys.exit(1)


def require_no_tag(tag: str) -> None:
    result = subprocess.run(
        ["git", "rev-parse", "-q", "--verify", f"refs/tags/{tag}"],
        cwd=ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    if result.returncode == 0:
        print(f"tag already exists locally: {tag}", file=sys.stderr)
        sys.exit(1)


def commit_report_if_changed(report: Path, tag: str) -> None:
    run(["git", "add", str(report.relative_to(ROOT))])
    staged = capture(["git", "diff", "--cached", "--name-only"])
    if str(report.relative_to(ROOT)) in staged.splitlines():
        run(["git", "commit", "-m", f"Add {tag} pentest report"])
    else:
        print(f"{report.relative_to(ROOT)} already committed.")


def validate_report_arg(name: str, value: str, pattern: str = r"^[^\n\r]+$") -> str:
    if re.fullmatch(pattern, value) is None:
        raise ValueError(f"--{name} contains invalid characters: {value!r}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Record pentest report, run release gate, commit, tag, and optionally push."
    )
    parser.add_argument(
        "--version",
        default=workspace_version(),
        help="Release version without leading v. Defaults to workspace version.",
    )
    parser.add_argument("--tester", required=True, help="Tester or review role.")
    parser.add_argument("--scope", required=True, help="Pentest scope.")
    parser.add_argument("--date", required=True, help="Pentest date in YYYY-MM-DD format.")
    parser.add_argument(
        "--push-main",
        action="store_true",
        help="Push main after committing the permanent pentest report.",
    )
    parser.add_argument(
        "--push-tag",
        action="store_true",
        help="Push the release tag after it is created.",
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="Skip the release version confirmation prompt.",
    )
    args = parser.parse_args()
    tester = validate_report_arg("tester", args.tester)
    scope = validate_report_arg("scope", args.scope)
    date = validate_report_arg("date", args.date, r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")

    tag = f"v{args.version}"
    version_parts = args.version.split(".")
    if len(version_parts) != 3:
        print("version must use X.Y.Z form", file=sys.stderr)
        return 1
    gate = f"scripts/release_{version_parts[0]}_{version_parts[1]}_gate.sh"
    scratch = ROOT / "PENTEST.md"
    report = ROOT / "security" / "pentest" / f"{tag}.md"

    if not scratch.is_file() and not report.is_file():
        print(
            f"missing root PENTEST.md scratch report and {report.relative_to(ROOT)}",
            file=sys.stderr,
        )
        return 1

    require_clean_release_tree(report)
    require_no_tag(tag)

    if not args.yes:
        answer = input(f"Type {args.version} to finalize {tag}: ").strip()
        if answer != args.version:
            print("version confirmation did not match; aborting", file=sys.stderr)
            return 1

    if scratch.is_file():
        run(
            [
                "scripts/record_pentest_report.py",
                "--version",
                args.version,
                "--tester",
                tester,
                "--scope",
                scope,
                "--date",
                date,
            ]
        )
        scratch.unlink()
    else:
        print(f"using existing {report.relative_to(ROOT)}")

    commit_report_if_changed(report, tag)
    run([gate])
    run(["git", "tag", "-a", tag, "-m", f"BCX {args.version}"])

    if args.push_main:
        run(["git", "push", "origin", "main"])
    if args.push_tag:
        run(["git", "push", "origin", tag])

    print(f"{tag} finalized.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
