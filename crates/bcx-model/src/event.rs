use bcx_core::{CapabilityRef, EventId, PolicyEpoch, ValidationError};

/// Relationship between a BCX event and one of its parents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipKind {
    /// The parent directly caused this event.
    CausedBy,
    /// The event was delegated by the parent.
    DelegatedFrom,
    /// The event retries the parent.
    RetryOf,
    /// The event was scheduled by the parent.
    ScheduledBy,
    /// The event was derived from the parent.
    DerivedFrom,
    /// The event joins several parent branches.
    JoinedFrom,
}

/// Observable cause class for an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CauseKind {
    /// An external network request entered the local trust boundary.
    ExternalRequest,
    /// A runtime or client attested an explicit user action.
    ExplicitUserAction,
    /// Application code initiated the operation.
    ApplicationAction,
    /// A service call was delegated by another participant.
    DelegatedServiceCall,
    /// A timer or schedule initiated the operation.
    Timer,
    /// A queue message initiated the operation.
    QueueMessage,
    /// The operation is a retry.
    Retry,
    /// An administrator initiated the operation.
    Administrator,
    /// An autonomous agent initiated the operation.
    AutonomousAgent,
}

/// High-level operation action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationAction {
    /// Read data without modifying authoritative state.
    Read,
    /// Create a new object or state transition.
    Create,
    /// Update existing state.
    Update,
    /// Delete or tombstone existing state.
    Delete,
    /// Derive an output from one or more inputs.
    Derive,
    /// Execute a component or tool.
    Execute,
    /// Transfer data or authority across a boundary.
    Transfer,
    /// Subscribe to future updates.
    Subscribe,
    /// Publish an event.
    Publish,
}

/// Admission decision produced before execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionResult {
    /// The operation may continue as requested.
    Allow,
    /// The operation is denied.
    Deny,
    /// The operation may continue only with a narrower scope.
    Narrow,
    /// The operation requires stronger approval.
    RequireApproval,
    /// The operation is quarantined for later review.
    Quarantine,
}

/// Execution result recorded after an operation attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectResult {
    /// The operation completed.
    Completed,
    /// The operation partially completed.
    Partial,
    /// The executor rejected the operation.
    Rejected,
    /// Execution failed.
    Failed,
    /// Execution was cancelled.
    Cancelled,
    /// Execution timed out.
    TimedOut,
}

/// Compact event capsule for causal parentage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CauseCapsule<'a> {
    /// Local event identifier.
    pub event_id: EventId,
    /// Parent event identifiers.
    pub parents: &'a [EventId],
    /// Relationship used for every parent in this compact capsule.
    pub relationship: RelationshipKind,
    /// Observable cause class.
    pub cause_kind: CauseKind,
    /// Requested action.
    pub action: OperationAction,
    /// Optional authority reference.
    pub authority: Option<CapabilityRef>,
    /// Optional policy epoch reference.
    pub policy_epoch: Option<PolicyEpoch>,
}

impl CauseCapsule<'_> {
    /// Validates bounded capsule shape.
    pub const fn validate(&self, maximum_parents: usize) -> Result<(), ValidationError> {
        if self.parents.len() > maximum_parents {
            Err(ValidationError::TooLarge)
        } else {
            Ok(())
        }
    }
}
