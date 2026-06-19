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
- Make variable-length identifier equality run the padded byte comparison even
  when identifier lengths differ.
- Store variable-length identifier lengths with checked conversion.
- Zeroize variable-length identifier backing storage on drop and avoid `Copy`
  for those identifier types.
- Remove `Hash` from `Nonce` so replay-cache implementations must choose a
  keyed or constant-time structure explicitly.
- Require non-empty `AlgorithmPolicy::new` input and add explicit
  `AlgorithmPolicy::deny_all`.
- Document that admitting multiple signature algorithms lets the sender choose
  the weakest admitted algorithm.
- Make hybrid verification invoke both signature component verifiers before
  combining the result.
- Rename development wire limits to
  `UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION`.
- Keep signature-envelope and wire-header revalidation internal to their
  defining crates.

## Known Limitations

- No statement body types yet.
- No canonical encoding yet.
- No real cryptographic provider yet.
- No capability verifier yet.
- No receipts or WHY bundles yet.
- `Digest` remains a copyable public commitment type and is not zeroized on
  drop. Sensitive private identifiers should use the variable-length
  identifier types added in this release or future dedicated secret wrappers.
- Expiry, audience binding, replay-cache hooks, and key-rotation evidence are
  still planned protocol controls and are not represented by `0.3.0` types.
