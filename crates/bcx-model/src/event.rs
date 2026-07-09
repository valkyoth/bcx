use bcx_core::{CapabilityRef, EventId, PolicyEpoch, ValidationError};
use bcx_wire::WireLimits;

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

/// Whether a referenced parent event is available to the local verifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentStatus {
    /// The parent event is available for verification or explanation.
    Present,
    /// The parent event is referenced but unavailable at this boundary.
    Missing,
}

impl ParentStatus {
    /// Returns true when the parent event is unavailable.
    #[must_use]
    pub const fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// One explicit causal edge from an event to a parent event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalEdge {
    parent: EventId,
    relationship: RelationshipKind,
    parent_status: ParentStatus,
}

impl CausalEdge {
    /// Creates an edge whose parent is available locally.
    #[must_use]
    pub const fn present(parent: EventId, relationship: RelationshipKind) -> Self {
        Self {
            parent,
            relationship,
            parent_status: ParentStatus::Present,
        }
    }

    /// Creates an edge whose parent is referenced but missing locally.
    #[must_use]
    pub const fn missing(parent: EventId, relationship: RelationshipKind) -> Self {
        Self {
            parent,
            relationship,
            parent_status: ParentStatus::Missing,
        }
    }

    /// Returns the parent event identifier.
    #[must_use]
    pub const fn parent(self) -> EventId {
        self.parent
    }

    /// Returns the relationship to the parent.
    #[must_use]
    pub const fn relationship(self) -> RelationshipKind {
        self.relationship
    }

    /// Returns the local parent availability status.
    #[must_use]
    pub const fn parent_status(self) -> ParentStatus {
        self.parent_status
    }
}

/// Caller-provided graph hook for cycle prevention beyond direct self-parent checks.
pub trait CausalCycleGuard {
    /// Returns true when adding `child -> parent` would create a causal cycle.
    fn would_create_cycle(&self, child: EventId, parent: EventId) -> bool;
}

/// Cycle guard for contexts that have no additional graph knowledge yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoKnownCycles;

impl CausalCycleGuard for NoKnownCycles {
    fn would_create_cycle(&self, _child: EventId, _parent: EventId) -> bool {
        false
    }
}

/// Input fields for a validated causal edge set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalEdgeSetParts<'a> {
    /// Local event identifier.
    pub event_id: EventId,
    /// Per-parent causal edges.
    pub edges: &'a [CausalEdge],
}

/// Validated causal edges for one event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CausalEdgeSet<'a> {
    event_id: EventId,
    edges: &'a [CausalEdge],
}

impl<'a> CausalEdgeSet<'a> {
    /// Creates a validated causal edge set with direct cycle checks.
    pub fn new(parts: CausalEdgeSetParts<'a>, limits: WireLimits) -> Result<Self, ValidationError> {
        Self::new_with_cycle_guard(parts, limits, &NoKnownCycles)
    }

    /// Creates a validated causal edge set using a caller-provided cycle guard.
    pub fn new_with_cycle_guard<G: CausalCycleGuard>(
        parts: CausalEdgeSetParts<'a>,
        limits: WireLimits,
        cycle_guard: &G,
    ) -> Result<Self, ValidationError> {
        let edge_set = Self {
            event_id: parts.event_id,
            edges: parts.edges,
        };
        edge_set.validate_with_cycle_guard(limits, cycle_guard)?;
        Ok(edge_set)
    }

    /// Validates bounded edge shape with direct cycle and duplicate checks.
    pub fn validate(&self, limits: WireLimits) -> Result<(), ValidationError> {
        self.validate_with_cycle_guard(limits, &NoKnownCycles)
    }

    /// Validates bounded edge shape and asks `cycle_guard` about graph cycles.
    pub fn validate_with_cycle_guard<G: CausalCycleGuard>(
        &self,
        limits: WireLimits,
        cycle_guard: &G,
    ) -> Result<(), ValidationError> {
        if self.edges.is_empty() {
            return Err(ValidationError::Empty);
        }
        if self.edges.len() > limits.maximum_parent_events() {
            return Err(ValidationError::TooLarge);
        }
        let mut index = 0;
        while index < self.edges.len() {
            let parent = self.edges[index].parent;
            if parent == self.event_id || cycle_guard.would_create_cycle(self.event_id, parent) {
                return Err(ValidationError::Malformed);
            }
            let mut duplicate_index = index + 1;
            while duplicate_index < self.edges.len() {
                if parent == self.edges[duplicate_index].parent {
                    return Err(ValidationError::Malformed);
                }
                duplicate_index += 1;
            }
            index += 1;
        }
        Ok(())
    }

    /// Returns the local event identifier.
    #[must_use]
    pub const fn event_id(&self) -> EventId {
        self.event_id
    }

    /// Returns validated causal edges.
    #[must_use]
    pub const fn edges(&self) -> &'a [CausalEdge] {
        self.edges
    }

    /// Returns true when at least one parent is referenced but missing locally.
    #[must_use]
    pub fn has_missing_parent(&self) -> bool {
        self.edges
            .iter()
            .any(|edge| edge.parent_status().is_missing())
    }
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
pub struct CauseCapsuleParts<'a> {
    /// Local event identifier.
    pub event_id: EventId,
    /// Parent event identifiers.
    ///
    /// The compact capsule uses the same [`RelationshipKind`] for every
    /// parent. Use separate capsules when parents need different relationship
    /// meanings.
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

/// Validated compact event capsule for causal parentage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CauseCapsule<'a> {
    event_id: EventId,
    parents: &'a [EventId],
    relationship: RelationshipKind,
    cause_kind: CauseKind,
    action: OperationAction,
    authority: Option<CapabilityRef>,
    policy_epoch: Option<PolicyEpoch>,
}

impl<'a> CauseCapsule<'a> {
    /// Creates a validated compact cause capsule.
    pub fn new(parts: CauseCapsuleParts<'a>, limits: WireLimits) -> Result<Self, ValidationError> {
        let capsule = Self {
            event_id: parts.event_id,
            parents: parts.parents,
            relationship: parts.relationship,
            cause_kind: parts.cause_kind,
            action: parts.action,
            authority: parts.authority,
            policy_epoch: parts.policy_epoch,
        };
        match capsule.validate(limits) {
            Ok(()) => Ok(capsule),
            Err(error) => Err(error),
        }
    }

    /// Validates bounded capsule shape and rejects direct self-parent cycles.
    pub fn validate(&self, limits: WireLimits) -> Result<(), ValidationError> {
        if self.parents.is_empty() {
            return Err(ValidationError::Empty);
        }
        if self.parents.len() > limits.maximum_parent_events() {
            return Err(ValidationError::TooLarge);
        }
        let mut index = 0;
        while index < self.parents.len() {
            let parent = self.parents[index];
            if parent == self.event_id {
                return Err(ValidationError::Malformed);
            }
            let mut duplicate_index = index + 1;
            while duplicate_index < self.parents.len() {
                if parent == self.parents[duplicate_index] {
                    return Err(ValidationError::Malformed);
                }
                duplicate_index += 1;
            }
            index += 1;
        }
        Ok(())
    }

    /// Returns the local event identifier.
    #[must_use]
    pub const fn event_id(&self) -> EventId {
        self.event_id
    }

    /// Returns parent event identifiers.
    #[must_use]
    pub const fn parents(&self) -> &'a [EventId] {
        self.parents
    }

    /// Returns the relationship used for each parent.
    #[must_use]
    pub const fn relationship(&self) -> RelationshipKind {
        self.relationship
    }

    /// Returns the observable cause class.
    #[must_use]
    pub const fn cause_kind(&self) -> CauseKind {
        self.cause_kind
    }

    /// Returns the requested action.
    #[must_use]
    pub const fn action(&self) -> OperationAction {
        self.action
    }

    /// Returns the optional authority reference.
    #[must_use]
    pub const fn authority(&self) -> Option<CapabilityRef> {
        self.authority
    }

    /// Returns the optional policy epoch reference.
    #[must_use]
    pub const fn policy_epoch(&self) -> Option<PolicyEpoch> {
        self.policy_epoch
    }
}
