#!/usr/bin/env bash
set -euo pipefail

required=(
    "README.md"
    "CHANGELOG.md"
    "LICENSE"
    "SECURITY.md"
    "docs/IMPLEMENTATION_PLAN.md"
    "docs/VERSION_PLAN.md"
    "docs/threat-model.md"
    "docs/modularity-policy.md"
    "docs/toolchain-policy.md"
    "release-notes/RELEASE_NOTES_0.1.0.md"
)

for path in "${required[@]}"; do
    if [ ! -s "$path" ]; then
        printf 'missing required release metadata: %s\n' "$path" >&2
        exit 1
    fi
done

if [ -e PENTEST.md ]; then
    printf 'PENTEST.md is scratch input and must not be committed\n' >&2
    exit 1
fi
