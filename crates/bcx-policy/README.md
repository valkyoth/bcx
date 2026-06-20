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
