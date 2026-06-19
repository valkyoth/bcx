# BCX 0.2.0 Release Notes

Status: unreleased

## Summary

BCX 0.2.0 pivots the project into a broader protocol family: one invariant
causal core with optional profiles, integrations, proof crates, domain
profiles, and services. It also hardens the release and publish gates after the
first crates.io publication.

## Changed

- Reframe BCX as Bifrost Causal Exchange: a semantic overlay protocol for
  signed causal meaning and proof composition across existing systems.
- Preserve the original idea draft as `docs/original-idea.md`.
- Add `docs/protocol-family.md`.
- Add repository family placeholders for future profiles, integrations, proof
  crates, domain profiles, and services.
- Rewrite the implementation plan around core, profile, integration, proof,
  domain, and service boundaries.
- Replace the version plan with a smaller-step core-first roadmap through
  release candidates before `1.0.0`.
- Clarify that `bcx-core`, `bcx-model`, `bcx-crypto`, `bcx-policy`, and
  `bcx-wire` remain valid foundation crates.
- Clarify that blockchain profiles such as Ethereum, Cardano, future Bitcoin,
  or future XRP are optional native bindings or settlement layers, not the BCX
  core.
- Split pre-tag release readiness from post-tag publish readiness.
- Require publish-time pentest metadata checks without rejecting an existing
  release tag.
- Align permanent pentest reports with the Aesynx-style release flow:
  `Tag`, `Commit`, `Status`, `Tester`, `Date`, and `Scope`.
- Accept pentest reports for current `HEAD` or `HEAD^` when current `HEAD` is
  only the permanent pentest report commit.
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
- Require hybrid Ed25519 plus ML-DSA-65 verification through explicit component
  verifier methods so both halves must verify before success.
- Document and expose the hybrid Ed25519 plus ML-DSA-65 signature split.
- Bound detached payload verification by `WireLimits`.
- Bind `CauseCapsule` parent limits to `WireLimits`.
- Sanitize pentest report metadata arguments.
- Make `cargo deny check` and `cargo audit` mandatory in the local check gate.
- Redact signature bytes from `SignatureEnvelope` debug output.
- Make `ProtocolVersion` fields private behind accessors.
- Avoid platform-dependent payload-length truncation in wire-header validation.
- Remove `Clone` from `Nonce` and document that `Hash` is not a replay-cache
  security boundary.

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
