# Changelog

All notable BCX changes will be documented here.

## 0.2.0 - Unreleased

- Split pre-tag release readiness from post-tag publish readiness.
- Add audited-commit and scratch-report digest checks for pentest reports.
- Add the `v0.2.0` release gate and publication metadata.
- Harden digest and nonce comparisons against byte-by-byte early exit.
- Reject all-zero nonces and clear nonce memory on drop with `zeroize`.
- Add operation-sequence successor helpers.
- Reject empty causal parent capsules.
- Make envelope, cause-capsule, and wire-header fields private behind
  validated constructors.
- Tie signature envelope bounds to `WireLimits`.
- Use fixed-width wire payload lengths.
- Warn on multiple dependency versions in `cargo-deny`.

## 0.1.0 - 2026-06-19

- Initialize the `bcx` Rust workspace.
- Add no-std root crate and focused internal crates.
- Add initial protocol identity, causal model, crypto envelope, policy, and wire limit primitives.
- Add security, release, implementation, and version planning documentation.
