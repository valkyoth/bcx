# Changelog

All notable BCX changes will be documented here.

## Unreleased

- Add a crate version matrix for future independent subcrate releases.
- Add a release gate that checks matrix entries against Cargo metadata and
  local path dependency version requirements.
- Make routine release-script checks stop assuming every workspace crate shares
  one package version.

## 0.2.0 - 2026-06-19

- Reframe BCX as Bifrost Causal Exchange: a semantic overlay protocol with one
  invariant core and optional native profiles.
- Preserve the new idea draft as `docs/original-idea.md`.
- Add `docs/protocol-family.md`.
- Add repository family placeholders for profiles, integrations, proofs,
  domain profiles, and services.
- Replace the roadmap with a smaller-step core-first version map.
- Split pre-tag release readiness from post-tag publish readiness.
- Align pentest report validation with the Aesynx-style `Tag`, `Commit`,
  `Status`, `Tester`, `Date`, and `Scope` flow.
- Accept a pentest report for `HEAD^` when `HEAD` only adds the permanent
  pentest report.
- Separate pentest report digestion from tag finalization.
- Make `finalize_release.py` tag-only after GitHub is green.
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
- Use `subtle` for constant-time digest and nonce equality.
- Remove ordering from digest and nonce-backed identifier types.
- Require explicit component verification for hybrid Ed25519 plus ML-DSA-65
  signatures.
- Document and expose the hybrid Ed25519 plus ML-DSA-65 signature split.
- Bound detached payload verification by `WireLimits`.
- Bind `CauseCapsule` parent limits to `WireLimits`.
- Sanitize pentest report metadata arguments.
- Confirm `cargo-deny 0.19.9` requires `all` for unmaintained and unsound advisory coverage.
- Require `cargo deny check` and `cargo audit` instead of skipping missing
  security tools.
- Redact signature bytes from `SignatureEnvelope` debug output.
- Make `ProtocolVersion` fields private.
- Avoid platform-dependent payload-length truncation in wire-header validation.
- Remove `Clone` from `Nonce` and document replay-cache hashing limitations.

## 0.1.0 - 2026-06-19

- Initialize the `bcx` Rust workspace.
- Add no-std root crate and focused internal crates.
- Add initial protocol identity, causal model, crypto envelope, policy, and wire limit primitives.
- Add security, release, implementation, and version planning documentation.
