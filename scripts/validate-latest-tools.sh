#!/usr/bin/env bash
set -euo pipefail

workflow=".github/workflows/ci.yml"
toolchain_doc="docs/toolchain-policy.md"

extract_crate_version() {
    crate="$1"
    cargo search "$crate" --limit 1 | awk -F '"' -v name="$crate" '$1 ~ "^" name " = " { print $2; exit }'
}

extract_pinned_tool_version() {
    tool="$1"
    sed -n "s/.*cargo install --locked ${tool}@\\([0-9][0-9.]*\\).*/\\1/p" "$workflow"
}

check_crate_tool() {
    tool="$1"
    latest="$(extract_crate_version "$tool")"
    pinned="$(extract_pinned_tool_version "$tool")"

    if [ -z "$latest" ]; then
        printf 'could not determine latest %s version from crates.io\n' "$tool" >&2
        exit 1
    fi
    if [ -z "$pinned" ]; then
        printf 'could not determine pinned %s version from %s\n' "$tool" "$workflow" >&2
        exit 1
    fi
    if [ "$latest" != "$pinned" ]; then
        printf '%s is stale: pinned %s, latest %s\n' "$tool" "$pinned" "$latest" >&2
        exit 1
    fi
}

latest_checkout_tag() {
    git ls-remote --tags https://github.com/actions/checkout.git \
        | sed -n 's#.*refs/tags/v\([0-9][0-9.]*\)$#\1#p' \
        | sort -V \
        | tail -n 1
}

checkout_sha_for_tag() {
    tag="$1"
    git ls-remote https://github.com/actions/checkout.git "refs/tags/v${tag}" \
        | awk '{ print $1; exit }'
}

check_checkout_action() {
    latest="$(latest_checkout_tag)"
    pinned_version="$(sed -n 's/.*actions\/checkout@.*# v\([0-9][0-9.]*\).*/\1/p' "$workflow")"
    pinned_sha="$(sed -n 's/.*actions\/checkout@\([0-9a-f]\{40\}\).*/\1/p' "$workflow")"

    if [ -z "$latest" ]; then
        printf 'could not determine latest actions/checkout tag\n' >&2
        exit 1
    fi
    if [ -z "$pinned_version" ] || [ -z "$pinned_sha" ]; then
        printf 'could not determine pinned actions/checkout version and SHA from %s\n' "$workflow" >&2
        exit 1
    fi
    if [ "$latest" != "$pinned_version" ]; then
        printf 'actions/checkout is stale: pinned v%s, latest v%s\n' "$pinned_version" "$latest" >&2
        exit 1
    fi

    latest_sha="$(checkout_sha_for_tag "$latest")"
    if [ -z "$latest_sha" ]; then
        printf 'could not determine actions/checkout v%s SHA\n' "$latest" >&2
        exit 1
    fi
    if [ "$latest_sha" != "$pinned_sha" ]; then
        printf 'actions/checkout v%s SHA mismatch: pinned %s, upstream %s\n' \
            "$latest" "$pinned_sha" "$latest_sha" >&2
        exit 1
    fi
}

check_toolchain_doc_mentions() {
    if ! rg -q 'cargo-deny 0\.19\.9' "$toolchain_doc"; then
        printf 'toolchain policy does not mention pinned cargo-deny version\n' >&2
        exit 1
    fi
    if ! rg -q 'cargo-audit 0\.22\.2' "$toolchain_doc"; then
        printf 'toolchain policy does not mention pinned cargo-audit version\n' >&2
        exit 1
    fi
    if ! rg -q 'actions/checkout v7\.0\.0' "$toolchain_doc"; then
        printf 'toolchain policy does not mention pinned actions/checkout version\n' >&2
        exit 1
    fi
}

check_crate_tool cargo-deny
check_crate_tool cargo-audit
check_checkout_action
check_toolchain_doc_mentions
