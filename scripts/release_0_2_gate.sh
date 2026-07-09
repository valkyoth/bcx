#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/validate-latest-tools.sh
scripts/release_crate.py --check
scripts/validate-release-readiness.sh v0.2.0

for toolchain in 1.90.0 1.91.0 1.92.0 1.93.0 1.94.0 1.95.0 1.96.0 1.96.1; do
    cargo "+$toolchain" check --workspace --all-features
done
