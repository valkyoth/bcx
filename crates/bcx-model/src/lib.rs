#![no_std]
#![doc = "Causal operation model types for BCX."]

mod event;
mod statement;
mod truth;

pub use event::{
    AdmissionResult, CauseCapsule, CauseCapsuleParts, CauseKind, EffectResult, OperationAction,
    RelationshipKind,
};
pub use statement::{
    Admission, Checkpoint, Contradiction, Delegation, Effect, Intent, Revocation, StatementKind,
};
pub use truth::{AssuranceLevel, TruthStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use bcx_core::{
        CapabilityRef, CheckpointId, Digest, EventId, PolicyId, StatementId, SubjectId,
        ValidationError,
    };
    use bcx_wire::WireLimits;

    fn event(byte: u8) -> Result<EventId, ValidationError> {
        EventId::new(Digest::new([byte; Digest::LEN]))
    }

    fn statement(byte: u8) -> Result<StatementId, ValidationError> {
        StatementId::new(&[byte; Digest::LEN])
    }

    fn subject(bytes: &[u8]) -> Result<SubjectId, ValidationError> {
        SubjectId::new(bytes)
    }

    fn policy(bytes: &[u8]) -> Result<PolicyId, ValidationError> {
        PolicyId::new(bytes)
    }

    fn capability(byte: u8) -> Result<CapabilityRef, ValidationError> {
        CapabilityRef::new(Digest::new([byte; Digest::LEN]))
    }

    fn checkpoint(byte: u8) -> Result<CheckpointId, ValidationError> {
        CheckpointId::new(&[byte; Digest::LEN])
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

    #[test]
    fn cause_capsule_rejects_self_referential_parent() -> Result<(), ValidationError> {
        let event_id = event(1)?;
        let parents = [event_id];

        assert_eq!(
            CauseCapsule::new(
                CauseCapsuleParts {
                    event_id,
                    parents: &parents,
                    relationship: RelationshipKind::CausedBy,
                    cause_kind: CauseKind::ApplicationAction,
                    action: OperationAction::Execute,
                    authority: None,
                    policy_epoch: None,
                },
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(ValidationError::Malformed)
        );
        Ok(())
    }

    #[test]
    fn statement_body_kinds_match_constructors() -> Result<(), ValidationError> {
        let intent = Intent::new(
            statement(1)?,
            subject(b"invoice:123")?,
            OperationAction::Create,
        );
        let admission = Admission::new(
            statement(2)?,
            statement(1)?,
            policy(b"strict")?,
            AdmissionResult::Allow,
        )?;
        let effect = Effect::new(statement(3)?, statement(1)?, EffectResult::Completed)?;
        let delegation = Delegation::new(
            statement(4)?,
            subject(b"issuer")?,
            subject(b"executor")?,
            capability(5)?,
        )?;
        let revocation = Revocation::new(statement(6)?, statement(4)?, capability(7)?)?;
        let checkpoint = Checkpoint::new(
            statement(8)?,
            checkpoint(9)?,
            subject(b"ledger:checkpoint")?,
        );
        let contradiction = Contradiction::new(statement(10)?, statement(2)?, statement(3)?)?;

        assert_eq!(intent.kind(), StatementKind::Intent);
        assert_eq!(admission.kind(), StatementKind::Admission);
        assert_eq!(effect.kind(), StatementKind::Effect);
        assert_eq!(delegation.kind(), StatementKind::Delegation);
        assert_eq!(revocation.kind(), StatementKind::Revocation);
        assert_eq!(checkpoint.kind(), StatementKind::Checkpoint);
        assert_eq!(contradiction.kind(), StatementKind::Contradiction);
        Ok(())
    }

    #[test]
    fn statement_body_validation_rejects_self_references() -> Result<(), ValidationError> {
        assert_eq!(
            Admission::new(
                statement(1)?,
                statement(1)?,
                policy(b"strict")?,
                AdmissionResult::Allow,
            ),
            Err(ValidationError::Malformed)
        );
        assert_eq!(
            Effect::new(statement(2)?, statement(2)?, EffectResult::Completed),
            Err(ValidationError::Malformed)
        );
        assert_eq!(
            Revocation::new(statement(3)?, statement(3)?, capability(4)?),
            Err(ValidationError::Malformed)
        );
        assert_eq!(
            Contradiction::new(statement(5)?, statement(6)?, statement(6)?),
            Err(ValidationError::Malformed)
        );
        Ok(())
    }

    #[test]
    fn delegation_rejects_same_subject() -> Result<(), ValidationError> {
        assert_eq!(
            Delegation::new(
                statement(1)?,
                subject(b"same")?,
                subject(b"same")?,
                capability(2)?,
            ),
            Err(ValidationError::Malformed)
        );
        Ok(())
    }
}
