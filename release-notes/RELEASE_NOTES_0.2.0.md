# BCX 0.2.0 Release Notes

Status: unreleased

## Summary

BCX 0.2.0 hardens the release and publish gates after the first crates.io
publication.

## Changed

- Split pre-tag release readiness from post-tag publish readiness.
- Require publish-time pentest metadata checks without rejecting an existing
  release tag.
- Record a SHA-256 digest of root `PENTEST.md` in the permanent pentest report.
- Require permanent pentest reports to record the exact audited commit.
- Reject release tags and publishes when code changed after the audited commit;
  only the permanent pentest report may differ.
- Add a release finalizer that records the pentest report, runs the release
  gate, commits the report, and creates the local tag.
- Allow the release finalizer to use an existing permanent pentest report when
  root `PENTEST.md` has already been removed.
- Add a dedicated `v0.2.0` release gate.
- Harden `Digest` and `Nonce` equality against byte-by-byte early exit.
- Reject all-zero nonces and clear nonce memory on drop with `zeroize`.
- Add `OperationSequence::next` and `OperationSequence::immediately_follows`.
- Reject empty `CauseCapsule` parent lists.
- Replace public struct-field construction with validated constructors for
  signature envelopes, cause capsules, signed envelopes, and wire headers.
- Tie signature envelope length checks to `WireLimits`.
- Use `u32` for wire payload lengths instead of platform-dependent `usize`.
- Warn on multiple dependency versions in `cargo-deny`.
- Confirm `cargo-deny 0.19.9` requires `all` for unmaintained and unsound
  advisory coverage; `deny` is not a valid value for those fields.
- Admit `zeroize 1.9.0` for no-std nonce clearing.
- Admit `subtle 2.6.1` for constant-time digest and nonce equality.
- Remove ordering from digest and nonce-backed identifier types.
- Document and expose the hybrid Ed25519 plus ML-DSA-65 signature split.
- Bound detached payload verification by `WireLimits`.
- Bind `CauseCapsule` parent limits to `WireLimits`.
- Sanitize pentest report metadata arguments.

## Deferred

- Signed pentest-report attestation requires auditor public-key provisioning and
  remains a future release-process hardening item.
- Typed canonical payload binding remains deferred until the canonical codec
  milestone; `verify_detached_bytes` now names the detached-byte contract
  explicitly.
- Audience binding, nonce expiry, replay cache hooks, and signing interfaces
  remain future protocol controls before production deployment.

## Known Limitations

- No canonical encoding yet.
- No real cryptographic provider yet.
- No capability verifier yet.
- No receipts or WHY bundles yet.
- No Fluxheim integration yet.
