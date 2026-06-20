use crate::{AdmissionResult, EffectResult, OperationAction};
use bcx_core::{CapabilityRef, CheckpointId, PolicyId, StatementId, SubjectId, ValidationError};

/// BCX statement body kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatementKind {
    /// A participant declares intended meaning before execution.
    Intent,
    /// A policy, runtime, or authority records an admission decision.
    Admission,
    /// A participant records the effect of an attempted operation.
    Effect,
    /// Authority or responsibility is delegated.
    Delegation,
    /// A prior statement, authority, or delegation is revoked.
    Revocation,
    /// A graph, state, or settlement checkpoint is recorded.
    Checkpoint,
    /// A statement contradicts another statement.
    Contradiction,
}

/// Declared intent before execution.
#[derive(Debug, Eq, PartialEq)]
pub struct Intent {
    statement_id: StatementId,
    subject: SubjectId,
    action: OperationAction,
}

impl Intent {
    /// Creates an intent body from required fields.
    #[must_use]
    pub const fn new(
        statement_id: StatementId,
        subject: SubjectId,
        action: OperationAction,
    ) -> Self {
        Self {
            statement_id,
            subject,
            action,
        }
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Intent
    }

    /// Returns the statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the subject identifier.
    #[must_use]
    pub const fn subject(&self) -> &SubjectId {
        &self.subject
    }

    /// Returns the requested action.
    #[must_use]
    pub const fn action(&self) -> OperationAction {
        self.action
    }
}

/// Admission decision for a prior intent.
#[derive(Debug, Eq, PartialEq)]
pub struct Admission {
    statement_id: StatementId,
    intent_id: StatementId,
    policy_id: PolicyId,
    result: AdmissionResult,
}

impl Admission {
    /// Creates an admission body from required fields.
    pub fn new(
        statement_id: StatementId,
        intent_id: StatementId,
        policy_id: PolicyId,
        result: AdmissionResult,
    ) -> Result<Self, ValidationError> {
        ensure_distinct(&statement_id, &intent_id)?;
        Ok(Self {
            statement_id,
            intent_id,
            policy_id,
            result,
        })
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Admission
    }

    /// Returns the admission statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the admitted intent identifier.
    #[must_use]
    pub const fn intent_id(&self) -> &StatementId {
        &self.intent_id
    }

    /// Returns the policy identifier used for the decision.
    #[must_use]
    pub const fn policy_id(&self) -> &PolicyId {
        &self.policy_id
    }

    /// Returns the admission result.
    #[must_use]
    pub const fn result(&self) -> AdmissionResult {
        self.result
    }
}

/// Recorded effect for a prior intent.
#[derive(Debug, Eq, PartialEq)]
pub struct Effect {
    statement_id: StatementId,
    intent_id: StatementId,
    result: EffectResult,
}

impl Effect {
    /// Creates an effect body from required fields.
    pub fn new(
        statement_id: StatementId,
        intent_id: StatementId,
        result: EffectResult,
    ) -> Result<Self, ValidationError> {
        ensure_distinct(&statement_id, &intent_id)?;
        Ok(Self {
            statement_id,
            intent_id,
            result,
        })
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Effect
    }

    /// Returns the effect statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the originating intent identifier.
    #[must_use]
    pub const fn intent_id(&self) -> &StatementId {
        &self.intent_id
    }

    /// Returns the effect result.
    #[must_use]
    pub const fn result(&self) -> EffectResult {
        self.result
    }
}

/// Delegation from one subject to another under a capability reference.
#[derive(Debug, Eq, PartialEq)]
pub struct Delegation {
    statement_id: StatementId,
    from_subject: SubjectId,
    to_subject: SubjectId,
    capability: CapabilityRef,
}

impl Delegation {
    /// Creates a delegation body from required fields.
    pub fn new(
        statement_id: StatementId,
        from_subject: SubjectId,
        to_subject: SubjectId,
        capability: CapabilityRef,
    ) -> Result<Self, ValidationError> {
        if from_subject == to_subject {
            return Err(ValidationError::Malformed);
        }
        Ok(Self {
            statement_id,
            from_subject,
            to_subject,
            capability,
        })
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Delegation
    }

    /// Returns the delegation statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the delegating subject.
    #[must_use]
    pub const fn from_subject(&self) -> &SubjectId {
        &self.from_subject
    }

    /// Returns the delegated subject.
    #[must_use]
    pub const fn to_subject(&self) -> &SubjectId {
        &self.to_subject
    }

    /// Returns the delegated capability reference.
    #[must_use]
    pub const fn capability(&self) -> CapabilityRef {
        self.capability
    }
}

/// Revocation of a prior statement by an authority.
#[derive(Debug, Eq, PartialEq)]
pub struct Revocation {
    statement_id: StatementId,
    target_id: StatementId,
    authority: CapabilityRef,
}

impl Revocation {
    /// Creates a revocation body from required fields.
    pub fn new(
        statement_id: StatementId,
        target_id: StatementId,
        authority: CapabilityRef,
    ) -> Result<Self, ValidationError> {
        ensure_distinct(&statement_id, &target_id)?;
        Ok(Self {
            statement_id,
            target_id,
            authority,
        })
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Revocation
    }

    /// Returns the revocation statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the revoked statement identifier.
    #[must_use]
    pub const fn target_id(&self) -> &StatementId {
        &self.target_id
    }

    /// Returns the revoking authority reference.
    #[must_use]
    pub const fn authority(&self) -> CapabilityRef {
        self.authority
    }
}

/// Checkpoint statement for committed graph, state, or settlement evidence.
#[derive(Debug, Eq, PartialEq)]
pub struct Checkpoint {
    statement_id: StatementId,
    checkpoint_id: CheckpointId,
    subject: SubjectId,
}

impl Checkpoint {
    /// Creates a checkpoint body from required fields.
    #[must_use]
    pub const fn new(
        statement_id: StatementId,
        checkpoint_id: CheckpointId,
        subject: SubjectId,
    ) -> Self {
        Self {
            statement_id,
            checkpoint_id,
            subject,
        }
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Checkpoint
    }

    /// Returns the checkpoint statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the checkpoint identifier.
    #[must_use]
    pub const fn checkpoint_id(&self) -> &CheckpointId {
        &self.checkpoint_id
    }

    /// Returns the checkpoint subject.
    #[must_use]
    pub const fn subject(&self) -> &SubjectId {
        &self.subject
    }
}

/// Contradiction between two distinct statements.
#[derive(Debug, Eq, PartialEq)]
pub struct Contradiction {
    statement_id: StatementId,
    disputed_id: StatementId,
    contradicts_id: StatementId,
}

impl Contradiction {
    /// Creates a contradiction body from required fields.
    pub fn new(
        statement_id: StatementId,
        disputed_id: StatementId,
        contradicts_id: StatementId,
    ) -> Result<Self, ValidationError> {
        ensure_distinct(&statement_id, &disputed_id)?;
        ensure_distinct(&statement_id, &contradicts_id)?;
        ensure_distinct(&disputed_id, &contradicts_id)?;
        Ok(Self {
            statement_id,
            disputed_id,
            contradicts_id,
        })
    }

    /// Returns this body kind.
    #[must_use]
    pub const fn kind(&self) -> StatementKind {
        StatementKind::Contradiction
    }

    /// Returns the contradiction statement identifier.
    #[must_use]
    pub const fn statement_id(&self) -> &StatementId {
        &self.statement_id
    }

    /// Returns the disputed statement identifier.
    #[must_use]
    pub const fn disputed_id(&self) -> &StatementId {
        &self.disputed_id
    }

    /// Returns the contradicting statement identifier.
    #[must_use]
    pub const fn contradicts_id(&self) -> &StatementId {
        &self.contradicts_id
    }
}

fn ensure_distinct(left: &StatementId, right: &StatementId) -> Result<(), ValidationError> {
    if left == right {
        Err(ValidationError::Malformed)
    } else {
        Ok(())
    }
}
