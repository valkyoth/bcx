# BCX 0.3.1 Release Notes

Status: unreleased

## Summary

BCX 0.3.1 is a documentation and package metadata release. It adds the BCX
README image, reshapes the root README after the sanitization crate
documentation model, and gives every current foundation subcrate its own small
README with examples.

## Changed

- Add the BCX README image under `.github/images/bcx.webp`.
- Rewrite the root README with status, trust dashboard, install, feature,
  workspace, example, verification, and security sections.
- Add packaged README files for `bcx-core`, `bcx-crypto`, `bcx-model`,
  `bcx-policy`, and `bcx-wire`.
- State clearly in each subcrate README that the subcrate belongs to the main
  `bcx` crate family and is not intended as a standalone protocol product.
- Move `bcx`, `bcx-core`, `bcx-crypto`, `bcx-model`, `bcx-policy`, and
  `bcx-wire` to package version `0.3.1`.

## Known Limitations

- This release does not add new protocol behavior.
- Optional companion crates such as future derive or external-integration
  crates remain future implementation work.
