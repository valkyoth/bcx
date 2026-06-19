# BCX 0.3.0 Release Notes

Status: unreleased

## Summary

BCX 0.3.0 starts the core statement foundation by adding the first no-std
statement identity vocabulary. It also makes the release tooling handle
selective subcrate publication so unchanged crates do not need a crates.io
upload for every repository tag.

## Added

- Add `StatementId`.
- Add `SubjectId`.
- Add `RealmId`.
- Add `ProfileId`.
- Add `ProofSuiteId`.
- Add `PolicyId`.
- Add `CheckpointId`.
- Add `NativeBindingId`.
- Add constructor tests for empty, malformed length, too-large length, and
  all-zero identifier values.
- Add a `v0.3.0` release gate.

## Changed

- Move `bcx-core`, `bcx-wire`, `bcx-crypto`, `bcx-model`, and root `bcx` to
  package version `0.3.0`.
- Keep `bcx-policy` at package version `0.2.0` because it has no `0.3.0`
  package-content changes and is outside the `bcx-core` dependency chain.
- Teach `release_crate.py` to publish only workspace crates whose package
  version matches the repository release being published.
- Teach the crate version matrix validator to ignore package-version metadata
  rewrites that do not change effective package content.

## Known Limitations

- No statement body types yet.
- No canonical encoding yet.
- No real cryptographic provider yet.
- No capability verifier yet.
- No receipts or WHY bundles yet.
