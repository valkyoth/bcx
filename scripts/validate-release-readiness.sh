#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 1 ]; then
    printf 'usage: %s <tag>\n' "$0" >&2
    exit 2
fi

tag="$1"
version="${tag#v}"
release_file="release-notes/RELEASE_NOTES_${version}.md"
pentest_file="security/pentest/${tag}.md"

if [ "$tag" = "$version" ]; then
    printf 'tag must start with v: %s\n' "$tag" >&2
    exit 1
fi

if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
    printf 'tag already exists locally: %s\n' "$tag" >&2
    exit 1
fi

if [ -e PENTEST.md ]; then
    printf 'PENTEST.md is scratch input and must not be committed\n' >&2
    exit 1
fi

if [ ! -s "$release_file" ]; then
    printf 'missing release notes: %s\n' "$release_file" >&2
    exit 1
fi

if [ ! -s "$pentest_file" ]; then
    printf 'missing pentest report: %s\n' "$pentest_file" >&2
    exit 1
fi

required_patterns=(
    '^Status: PASS$'
    '^Commit: [0-9a-fA-F]{40}$'
    '^Tester: .+'
    '^Scope: .+'
    '^Date: [0-9]{4}-[0-9]{2}-[0-9]{2}$'
)

for pattern in "${required_patterns[@]}"; do
    if ! grep -Eq "$pattern" "$pentest_file"; then
        printf 'pentest report %s missing required pattern: %s\n' \
            "$pentest_file" "$pattern" >&2
        exit 1
    fi
done

reported_commit="$(sed -n 's/^Commit: //p' "$pentest_file" | head -n 1)"
head_commit="$(git rev-parse HEAD)"
if [ "$reported_commit" != "$head_commit" ]; then
    printf 'pentest report %s commit %s does not match HEAD %s\n' \
        "$pentest_file" "$reported_commit" "$head_commit" >&2
    exit 1
fi
