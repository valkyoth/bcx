# BCX 0.1.0 Release Notes

Status: unreleased

## Summary

BCX 0.1.0 establishes the repository, workspace, security posture, and first
no-std protocol primitives.

## Added

- Root published crate `bcx`.
- Focused internal crates:
  - `bcx-core`
  - `bcx-model`
  - `bcx-crypto`
  - `bcx-policy`
  - `bcx-wire`
- EUPL-1.2 licensing.
- Local check script.
- Version-specific release gate with latest-tool validation before tagging.
- Security, implementation, version, threat-model, toolchain, modularity, and
  supply-chain documentation.

## Known Limitations

- No canonical encoding yet.
- No real cryptographic provider yet.
- No capability verifier yet.
- No receipts or WHY bundles yet.
- No Fluxheim integration yet.
