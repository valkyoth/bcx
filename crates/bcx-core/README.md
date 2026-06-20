# bcx-core

Core identifier and validation primitives for the main
[`bcx`](https://crates.io/crates/bcx) crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_core::{Digest, StatementId, SubjectId};

let statement = StatementId::new(&[7; Digest::LEN]).unwrap();
let subject = SubjectId::new(b"subject:invoice:123").unwrap();

assert_eq!(statement.len(), Digest::LEN);
assert_eq!(subject.as_bytes(), b"subject:invoice:123");
```

## Notes

- `no_std` by default.
- Raw identifier bytes are validated before construction.
- Debug output is intentionally redacted for byte-backed identifiers.
