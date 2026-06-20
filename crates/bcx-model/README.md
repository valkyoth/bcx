# bcx-model

Causal model primitives for the main [`bcx`](https://crates.io/crates/bcx)
crate.

This crate belongs to the BCX workspace. It is published separately so the main
crate can keep small internal modules and precise dependency boundaries, but it
is not meant to be used as a standalone protocol product. Prefer depending on
`bcx` unless you are working on BCX internals.

## Example

```rust
use bcx_core::{Digest, EventId};
use bcx_model::{
    CauseCapsule, CauseCapsuleParts, CauseKind, OperationAction, RelationshipKind,
};
use bcx_wire::WireLimits;

let event_id = EventId::new(Digest::new([1; Digest::LEN])).unwrap();
let parent = EventId::new(Digest::new([2; Digest::LEN])).unwrap();
let parents = [parent];

let capsule = CauseCapsule::new(
    CauseCapsuleParts {
        event_id,
        parents: &parents,
        relationship: RelationshipKind::CausedBy,
        cause_kind: CauseKind::ApplicationAction,
        action: OperationAction::Execute,
        authority: None,
        policy_epoch: None,
    },
    WireLimits::new(64 * 1024, 16, 5, 100).unwrap(),
)
.unwrap();

assert_eq!(capsule.parents().len(), 1);
```

## Notes

- `no_std` by default.
- Model constructors enforce current BCX shape and bound rules.
- Canonical encoding and full statement bodies are future milestones.
