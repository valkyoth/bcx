#![no_std]
#![doc = "Causal operation model types for BCX."]

mod event;
mod truth;

pub use event::{
    AdmissionResult, CauseCapsule, CauseKind, EffectResult, OperationAction, RelationshipKind,
};
pub use truth::{AssuranceLevel, TruthStatus};
