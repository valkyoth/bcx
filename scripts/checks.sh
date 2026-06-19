#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all --check
cargo test --workspace
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

target_installed() {
    rustup target list --installed | grep -Fxq "$1"
}

check_installed_target() {
    target="$1"
    shift

    if target_installed "$target"; then
        cargo check --workspace --target "$target" "$@"
    else
        printf 'skipping target check for %s; target is not installed\n' "$target"
    fi
}

check_installed_target x86_64-unknown-linux-gnu --all-features
check_installed_target aarch64-unknown-linux-gnu --all-features
check_installed_target x86_64-apple-darwin --all-features
check_installed_target aarch64-apple-darwin --all-features
check_installed_target aarch64-apple-ios --all-features
check_installed_target x86_64-apple-ios --all-features
check_installed_target x86_64-pc-windows-gnu --all-features
check_installed_target aarch64-linux-android --all-features
check_installed_target x86_64-linux-android --all-features
check_installed_target x86_64-unknown-freebsd --all-features
check_installed_target wasm32-unknown-unknown --no-default-features
check_installed_target wasm32-unknown-unknown --all-features
check_installed_target wasm32-wasip1 --all-features
check_installed_target wasm32-wasip2 --all-features
check_installed_target thumbv7em-none-eabihf --no-default-features

scripts/validate-modularity-policy.sh
scripts/validate-release-metadata.sh
scripts/validate-latest-tools.sh
scripts/release_crate.py --check

cargo package -p bcx-core --allow-dirty --list >/dev/null
cargo package -p bcx-crypto --allow-dirty --list >/dev/null
cargo package -p bcx-model --allow-dirty --list >/dev/null
cargo package -p bcx-policy --allow-dirty --list >/dev/null
cargo package -p bcx-wire --allow-dirty --list >/dev/null
cargo package -p bcx --allow-dirty --list >/dev/null

if cargo deny --version >/dev/null 2>&1; then
    cargo deny check
else
    printf 'skipping cargo deny; cargo-deny is not installed\n'
fi

if cargo audit --version >/dev/null 2>&1; then
    cargo audit
else
    printf 'skipping cargo audit; cargo-audit is not installed\n'
fi
