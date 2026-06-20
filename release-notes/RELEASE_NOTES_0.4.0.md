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

## Known Limitations

- No canonical encoding yet.
- No statement envelope yet.
- No signatures or attestations are bound to statement bodies yet.
- No real cryptographic provider yet.
- Optional companion crates such as future derive or external-integration
  crates remain future implementation work.
