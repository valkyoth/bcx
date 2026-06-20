# bcx-crypto

Crypto-agile signature envelope metadata for the main
[`bcx`](https://crates.io/crates/bcx) crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_core::Digest;
use bcx_crypto::{ExactAlgorithmPolicy, SignatureAlgorithm, SignatureEnvelope};
use bcx_wire::WireLimits;

let signature = [7; SignatureAlgorithm::ED25519_SIGNATURE_LEN];
let limits = WireLimits::new(64 * 1024, 16, 5, 100).unwrap();
let envelope = SignatureEnvelope::new(
    Digest::new([1; Digest::LEN]),
    SignatureAlgorithm::Ed25519,
    &signature,
    limits,
)
.unwrap();
let policy = ExactAlgorithmPolicy::new(SignatureAlgorithm::Ed25519);

assert!(policy.admits(envelope.algorithm()));
```

## Notes

- `no_std` by default.
- This crate defines metadata and verifier traits, not cryptographic provider
  implementations.
- Hybrid signature verification requires both components to be checked.
