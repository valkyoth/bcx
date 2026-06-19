#![no_std]
#![doc = "Causal operation model types for BCX."]

mod event;
mod truth;

pub use event::{
    AdmissionResult, CauseCapsule, CauseCapsuleParts, CauseKind, EffectResult, OperationAction,
    RelationshipKind,
};
pub use truth::{AssuranceLevel, TruthStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use bcx_core::{Digest, EventId, ValidationError};
    use bcx_wire::WireLimits;

    fn event(byte: u8) -> Result<EventId, ValidationError> {
        EventId::new(Digest::new([byte; Digest::LEN]))
    }

    #[test]
    fn cause_capsule_rejects_empty_parents() -> Result<(), ValidationError> {
        assert_eq!(
            CauseCapsule::new(
                CauseCapsuleParts {
                    event_id: event(1)?,
                    parents: &[],
                    relationship: RelationshipKind::CausedBy,
                    cause_kind: CauseKind::ApplicationAction,
                    action: OperationAction::Execute,
                    authority: None,
                    policy_epoch: None,
                },
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(ValidationError::Empty)
        );
        Ok(())
    }

    #[test]
    fn cause_capsule_rejects_too_many_parents() -> Result<(), ValidationError> {
        let parents = [event(2)?, event(3)?];
        let limits = WireLimits::new(1, 1, 1, 1)?;

        assert_eq!(
            CauseCapsule::new(
                CauseCapsuleParts {
                    event_id: event(1)?,
                    parents: &parents,
                    relationship: RelationshipKind::JoinedFrom,
                    cause_kind: CauseKind::ApplicationAction,
                    action: OperationAction::Execute,
                    authority: None,
                    policy_epoch: None,
                },
                limits,
            ),
            Err(ValidationError::TooLarge)
        );
        Ok(())
    }
}
