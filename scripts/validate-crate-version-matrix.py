#!/usr/bin/env python3
"""Validate BCX crate version metadata."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from collections import deque
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - release host guard.
    print("Python 3.11+ is required because this script uses tomllib.", file=sys.stderr)
    raise


ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "CRATE_VERSION_MATRIX.md"


def capture(command: list[str]) -> str:
    return subprocess.check_output(command, cwd=ROOT, text=True).strip()


def cargo_metadata() -> dict:
    return json.loads(capture(["cargo", "metadata", "--no-deps", "--format-version", "1"]))


def workspace_packages() -> dict[str, dict[str, str]]:
    root = str(ROOT)
    packages = {}
    for package in cargo_metadata()["packages"]:
        manifest_path = package["manifest_path"]
        if manifest_path.startswith(root):
            rel_manifest = Path(manifest_path).relative_to(ROOT)
            rel_dir = rel_manifest.parent
            packages[package["name"]] = {
                "version": package["version"],
                "path": "." if str(rel_dir) == "." else str(rel_dir),
                "manifest": str(rel_manifest),
            }
    return packages


def parse_matrix() -> dict[str, dict[str, str]]:
    text = MATRIX.read_text(encoding="utf-8")
    rows = {}
    for line in text.splitlines():
        if not line.startswith("| `"):
            continue
        columns = [column.strip() for column in line.strip("|").split("|")]
        if len(columns) < 5:
            continue
        name = columns[0].strip("`")
        rows[name] = {
            "path": columns[1].strip("`"),
            "version": columns[2].strip("`"),
        }
    return rows


def read_toml_at(path: Path) -> dict:
    with path.open("rb") as handle:
        return tomllib.load(handle)


def local_path_dependencies(manifest: Path) -> list[tuple[str, str, str]]:
    data = read_toml_at(manifest)
    dependencies = data.get("dependencies", {})
    found = []
    for name, spec in dependencies.items():
        if not isinstance(spec, dict) or "path" not in spec:
            continue
        version = spec.get("version")
        if not isinstance(version, str):
            raise RuntimeError(f"{manifest.relative_to(ROOT)} dependency {name} lacks version")
        dep_path = (manifest.parent / spec["path"]).resolve()
        dep_manifest = (dep_path / "Cargo.toml").relative_to(ROOT)
        found.append((name, str(dep_manifest), version))
    return found


def latest_tag() -> str | None:
    result = subprocess.run(
        ["git", "describe", "--tags", "--abbrev=0"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return result.stdout.strip()


def changed_paths_since(tag: str) -> set[str]:
    output = capture(["git", "diff", "--name-only", f"{tag}..HEAD"])
    return {line for line in output.splitlines() if line}


def file_at_tag(tag: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "show", f"{tag}:{path}"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return result.stdout


def workspace_version_at(tag: str) -> str | None:
    text = file_at_tag(tag, "Cargo.toml")
    if text is None:
        return None
    return first_version_value(text, r"^\[workspace\.package\]$")


def first_version_value(text: str, section_pattern: str | None = None) -> str | None:
    in_section = section_pattern is None
    for line in text.splitlines():
        if section_pattern is not None and re.match(section_pattern, line):
            in_section = True
            continue
        if in_section and line.startswith("[") and not re.match(section_pattern or "$^", line):
            return None
        if in_section and line.startswith("version = "):
            return line.split('"', 2)[1]
    return None


def package_version_at(tag: str, manifest: str) -> str | None:
    text = file_at_tag(tag, manifest)
    if text is None:
        return None
    package_version = first_version_value(text, r"^\[package\]$")
    if package_version is not None:
        return package_version
    if "version.workspace = true" in text:
        return workspace_version_at(tag)
    return None


def normalized_manifest(text: str) -> str:
    lines = []
    in_package = False
    in_workspace_package = False

    for line in text.splitlines():
        if line == "[package]":
            in_package = True
            in_workspace_package = False
            lines.append(line)
            continue
        if line == "[workspace.package]":
            in_package = False
            in_workspace_package = True
            lines.append(line)
            continue
        if line.startswith("["):
            in_package = False
            in_workspace_package = False
            lines.append(line)
            continue
        if in_package and (line.startswith("version = ") or line == "version.workspace = true"):
            continue
        if in_workspace_package and line.startswith("version = "):
            continue
        lines.append(line)

    return "\n".join(lines)


def manifest_content_changed_since(tag: str, manifest: str) -> bool:
    old_text = file_at_tag(tag, manifest)
    if old_text is None:
        return True
    current_text = (ROOT / manifest).read_text(encoding="utf-8")
    return normalized_manifest(old_text) != normalized_manifest(current_text)


def package_content_changed(path: str, manifest: str, tag: str, changed: set[str]) -> bool:
    if path == ".":
        prefixes = ("src/",)
        manifest_path = "Cargo.toml"
    else:
        prefixes = (f"{path}/src/",)
        manifest_path = f"{path}/Cargo.toml"

    if any(item.startswith(prefixes) for item in changed):
        return True
    if manifest_path in changed:
        return manifest_content_changed_since(tag, manifest)
    return False


def package_version_changed_since(tag: str, package: dict[str, str]) -> bool:
    old_version = package_version_at(tag, package["manifest"])
    return old_version != package["version"]


def dependency_bump_requirements(
    changed_versions: set[str], dependents_by_dep: dict[str, set[str]]
) -> dict[str, set[str]]:
    required_by: dict[str, set[str]] = {}
    queue = deque(sorted(changed_versions))

    while queue:
        dependency = queue.popleft()
        for dependent in sorted(dependents_by_dep.get(dependency, set())):
            reasons = required_by.setdefault(dependent, set())
            before = len(reasons)
            reasons.add(dependency)
            if len(reasons) != before and dependent not in changed_versions:
                queue.append(dependent)

    return required_by


def validate() -> None:
    packages = workspace_packages()
    matrix = parse_matrix()

    if set(packages) != set(matrix):
        missing = sorted(set(packages) - set(matrix))
        extra = sorted(set(matrix) - set(packages))
        raise RuntimeError(f"crate version matrix mismatch: missing={missing}, extra={extra}")

    manifest_to_package = {
        package["manifest"]: (name, package["version"]) for name, package in packages.items()
    }
    dependents_by_dep: dict[str, set[str]] = {}

    for name, package in sorted(packages.items()):
        row = matrix[name]
        if row["path"] != package["path"]:
            raise RuntimeError(f"{name} matrix path {row['path']} != {package['path']}")
        if row["version"] != package["version"]:
            raise RuntimeError(
                f"{name} matrix version {row['version']} != Cargo version {package['version']}"
            )

        manifest = ROOT / package["manifest"]
        for dep_name, dep_manifest, required_version in local_path_dependencies(manifest):
            if dep_manifest not in manifest_to_package:
                continue
            actual_name, actual_version = manifest_to_package[dep_manifest]
            dependents_by_dep.setdefault(actual_name, set()).add(name)
            if required_version != actual_version:
                raise RuntimeError(
                    f"{name} dependency {dep_name} requires {required_version}, "
                    f"but local {actual_name} is {actual_version}"
                )

    tag = latest_tag()
    if tag is None:
        print("crate version matrix: ok (no release tag found)")
        return

    changed = changed_paths_since(tag)
    old_versions = {}
    changed_versions = set()
    for name, package in sorted(packages.items()):
        old_version = package_version_at(tag, package["manifest"])
        old_versions[name] = old_version
        if package_version_changed_since(tag, package):
            changed_versions.add(name)

        if not package_content_changed(package["path"], package["manifest"], tag, changed):
            continue
        if old_version == package["version"]:
            raise RuntimeError(
                f"{name} package content changed since {tag} but version is still "
                f"{package['version']}"
            )

    required_bumps = dependency_bump_requirements(changed_versions, dependents_by_dep)
    for name, dependencies in sorted(required_bumps.items()):
        if old_versions.get(name) != packages[name]["version"]:
            continue
        dependency_list = ", ".join(f"`{dependency}`" for dependency in sorted(dependencies))
        raise RuntimeError(
            f"{name} must bump because local dependency version changed since {tag}: "
            f"{dependency_list}"
        )

    print("crate version matrix: ok")


def main() -> int:
    try:
        validate()
    except RuntimeError as exc:
        print(f"crate version matrix: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
