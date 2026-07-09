<p align="center">
  <b>validated no_std identifier primitives for bcx.</b><br>
  Digests, nonces, sequence numbers, redacted debug output, and constant-shape comparisons.
</p>

<div align="center">
  <a href="https://crates.io/crates/bcx">bcx crate</a>
  |
  <a href="https://docs.rs/bcx-core">Docs.rs</a>
  |
  <a href="https://github.com/valkyoth/bcx/blob/main/docs/VERSION_PLAN.md">Version Plan</a>
  |
  <a href="https://github.com/valkyoth/bcx/blob/main/docs/threat-model.md">Threat Model</a>
  |
  <a href="https://github.com/valkyoth/bcx/blob/main/SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <a href="https://github.com/valkyoth/bcx">
    <img src="https://raw.githubusercontent.com/valkyoth/bcx/main/.github/images/bcx.webp" alt="BCX Rust crate overview">
  </a>
</p>

# bcx-core

Core identifier and validation primitives for the main
[`bcx`](https://crates.io/crates/bcx) crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_core::{Digest, StatementId, SubjectId, ZeroizedDigest};

let statement = StatementId::new(&[7; Digest::LEN]).unwrap();
let subject = SubjectId::new(b"subject:invoice:123").unwrap();
let boundary_digest = ZeroizedDigest::new(Digest::new([9; Digest::LEN]));

assert_eq!(statement.len(), Digest::LEN);
assert_eq!(subject.as_bytes(), b"subject:invoice:123");
assert_eq!(boundary_digest.as_bytes(), &[9; Digest::LEN]);
```

## Notes

- `no_std` by default.
- Raw identifier bytes are validated before construction.
- Debug output is intentionally redacted for byte-backed identifiers.
