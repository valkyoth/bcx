# BCX 0.2.0 Release Notes

Status: unreleased

## Summary

BCX 0.2.0 hardens the release and publish gates after the first crates.io
publication.

## Changed

- Split pre-tag release readiness from post-tag publish readiness.
- Require publish-time pentest metadata checks without rejecting an existing
  release tag.
- Require pentest reports to point at the exact commit being released.
- Add a dedicated `v0.2.0` release gate.

## Known Limitations

- No canonical encoding yet.
- No real cryptographic provider yet.
- No capability verifier yet.
- No receipts or WHY bundles yet.
- No Fluxheim integration yet.
