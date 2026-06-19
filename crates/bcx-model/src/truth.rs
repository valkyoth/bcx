/// Truth status attached to explanation claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TruthStatus {
    /// A local component directly observed this fact.
    Observed,
    /// A participant declared this claim.
    Declared,
    /// A signature, capability, or proof validated for this claim.
    Verified,
    /// A named policy controlled this decision.
    Enforced,
    /// A verifier assessed runtime or hardware evidence.
    Attested,
    /// Another participant signed its view of the event.
    Acknowledged,
    /// An independent service recorded a commitment.
    Witnessed,
    /// No reliable evidence is available.
    Unknown,
}

/// Assurance level for a causal edge or effect claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum AssuranceLevel {
    /// One participant claimed the edge.
    ClaimedBySender,
    /// The receiver observed the invocation.
    ObservedByReceiver,
    /// Sender and receiver cross-acknowledged the edge.
    CrossAcknowledged,
    /// The operation effect was confirmed by a responsible component.
    EffectConfirmed,
    /// An independent witness recorded a commitment.
    IndependentlyWitnessed,
}
