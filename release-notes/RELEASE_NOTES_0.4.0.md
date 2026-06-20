# BCX 0.4.0 Release Notes

Status: unreleased

## Summary

BCX 0.4.0 adds the first BCX/1 statement body vocabulary without encoding or
signatures. It also carries the README and package metadata improvements that
landed after `v0.3.0`.

## Added

- Add `StatementKind`.
- Add `Intent`.
- Add `Admission`.
- Add `Effect`.
- Add `Delegation`.
- Add `Revocation`.
- Add `Checkpoint`.
- Add `Contradiction`.
- Add minimum required-field validation tests for self-referential statement
  links and same-subject delegation.
- Add `ZeroizedDigest` for digest values held at processing boundaries where
  memory residue is part of the threat model.
- Add `ExactAlgorithmPolicy` and exact-policy detached verification for
  high-assurance contexts that must avoid sender-choice algorithm downgrade.
- Add the BCX README image under `.github/images/bcx.webp`.
- Add packaged README files for `bcx-core`, `bcx-crypto`, `bcx-model`,
  `bcx-policy`, and `bcx-wire`.

## Changed

- Rewrite the root README with status, trust dashboard, install, feature,
  workspace, example, verification, and security sections.
- State clearly in each subcrate README that the subcrate belongs to the main
  `bcx` crate family and is not intended as a standalone protocol product.
- Move `bcx`, `bcx-core`, `bcx-crypto`, `bcx-model`, `bcx-policy`, and
  `bcx-wire` to package version `0.4.0`.
- `CauseCapsule` validation now rejects direct self-parent cycles.
- `CauseCapsuleParts` documentation now states that one relationship kind
  applies to every parent in the compact capsule.
- `Digest` documentation now points sensitive-boundary users to
  `ZeroizedDigest`.
- `HybridVerifier` documentation now keeps the no-short-circuit verification
  requirement visible for custom backends.
- `ZeroizedDigest::digest()` documentation now warns that the returned `Digest`
  copy is not zeroed on drop.
- `ZeroizedDigest` is now exported from the root `bcx::prelude`.

## Security Review

The v0.4.0 pentest findings were handled as follows:

- `MEDIUM-1`: added `ZeroizedDigest` instead of removing `Copy` from `Digest`
  in a pre-1.0 breaking surface.
- `MEDIUM-2`: fixed by rejecting self-referential `CauseCapsule` parents.
- `MEDIUM-3`: added `ExactAlgorithmPolicy` and exact-policy verification.
- `LOW-1`, `LOW-2`, and `LOW-3`: tracked in the version plan before the
  affected statement-envelope, replay, and verification milestones.
- `INFORMATIONAL-1`: documented the compact capsule relationship constraint.
- `INFORMATIONAL-2`: kept the hybrid verifier no-short-circuit rule prominent
  in the trait contract.
- Follow-up `LOW-1`: fixed with explicit copy-hazard documentation on
  `ZeroizedDigest::digest()`.
- Follow-up `LOW-2`: fixed by exporting `ZeroizedDigest` from
  `bcx::prelude`.

## Known Limitations

- No canonical encoding yet.
- No statement envelope yet.
- No signatures or attestations are bound to statement bodies yet.
- No real cryptographic provider yet.
- Freshness, audience binding, and replay cache traits remain scheduled before
  production carrier profiles.
- Optional companion crates such as future derive or external-integration
  crates remain future implementation work.
