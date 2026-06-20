# bcx-wire

Wire version and bounded-message primitives for the main
[`bcx`](https://crates.io/crates/bcx) crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_wire::{ProtocolVersion, WireHeader, WireLimits};

let limits = WireLimits::new(64 * 1024, 16, 5, 100).unwrap();
let header = WireHeader::new(ProtocolVersion::CURRENT, 4096, limits).unwrap();

assert_eq!(header.version().major(), 1);
assert_eq!(header.payload_len(), 4096);
```

## Notes

- `no_std` by default.
- Production profiles should construct explicit `WireLimits`.
- `UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION` exists only for tests and
  development examples.
