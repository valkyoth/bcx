<p align="center">
  <b>policy vocabulary primitives for bcx.</b><br>
  Protocol profiles, proof levels, disclosure levels, replay policy, and settlement intent.
</p>

<div align="center">
  <a href="https://crates.io/crates/bcx">bcx crate</a>
  |
  <a href="https://docs.rs/bcx-policy">Docs.rs</a>
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

# bcx-policy

Policy vocabulary primitives for the main [`bcx`](https://crates.io/crates/bcx)
crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_policy::{DisclosureLevel, ProofLevel, ProtocolProfile};

let profile = ProtocolProfile::Sovereign;
let proof = ProofLevel::ExplicitUserApproval;
let disclosure = DisclosureLevel::HashCommitment;

assert!(profile.requires_signed_consequential_invocations());
assert!(profile.forbids_state_changing_early_data());
assert!(proof >= ProofLevel::RuntimeAttested);
assert_eq!(disclosure, DisclosureLevel::HashCommitment);
```

## Notes

- `no_std` by default.
- This crate names policy vocabulary; enforcement is performed by BCX users,
  profiles, and future verifier layers.
- The vocabulary is intentionally conservative until profile contracts mature.
