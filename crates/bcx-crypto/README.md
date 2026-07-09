<p align="center">
  <b>crypto-agile envelope metadata for bcx.</b><br>
  Signature algorithms, exact admission policy, verifier traits, and hybrid-proof boundaries.
</p>

<div align="center">
  <a href="https://crates.io/crates/bcx">bcx crate</a>
  |
  <a href="https://docs.rs/bcx-crypto">Docs.rs</a>
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
