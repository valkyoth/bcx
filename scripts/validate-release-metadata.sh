#!/usr/bin/env bash
set -euo pipefail

required=(
    "README.md"
    ".github/images/bcx.webp"
    "CHANGELOG.md"
    "LICENSE"
    "SECURITY.md"
    "docs/IMPLEMENTATION_PLAN.md"
    "docs/VERSION_PLAN.md"
    "docs/CRATE_VERSION_MATRIX.md"
    "docs/protocol-family.md"
    "docs/original-idea.md"
    "docs/threat-model.md"
    "docs/modularity-policy.md"
    "docs/toolchain-policy.md"
    "crates/README.md"
    "profiles/README.md"
    "integrations/README.md"
    "proofs/README.md"
    "domains/README.md"
    "services/README.md"
    "release-notes/RELEASE_NOTES_0.1.0.md"
    "release-notes/RELEASE_NOTES_0.2.0.md"
    "release-notes/RELEASE_NOTES_0.3.0.md"
    "release-notes/RELEASE_NOTES_0.4.0.md"
    "scripts/release_0_1_gate.sh"
    "scripts/release_0_2_gate.sh"
    "scripts/release_0_3_gate.sh"
    "scripts/release_0_4_gate.sh"
    "scripts/validate-release-readiness.sh"
    "scripts/validate-crate-version-matrix.py"
    "scripts/validate-latest-tools.sh"
    "scripts/record_pentest_report.py"
    "scripts/finalize_release.py"
    "scripts/release_crate.py"
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
