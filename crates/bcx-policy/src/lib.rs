#![no_std]
#![doc = "Security profile and disclosure primitives for BCX."]

/// Deployment profile for BCX verification strictness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolProfile {
    /// Hash-chained local events with periodic commitment.
    Audit,
    /// Individually authenticated cross-boundary events.
    Federated,
    /// Strict high-assurance profile for sensitive operations.
    Sovereign,
}

/// Proof strength attached to a root cause or user action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum ProofLevel {
    /// No actor attribution is carried.
    Unattributed,
    /// A runtime observed the event.
    RuntimeAttested,
    /// A pairwise pseudonymous identity is used.
    Pseudonymous,
    /// An account-authenticated identity is used.
    AccountAuthenticated,
    /// A visible explicit approval was captured.
    ExplicitUserApproval,
    /// A formal legal signature was captured.
    LegalSignature,
}

/// Disclosure level for a field in an explanation bundle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisclosureLevel {
    /// Disclose the field as plaintext.
    Plaintext,
    /// Disclose encrypted material for authorized readers.
    Encrypted,
    /// Disclose only a hash commitment.
    HashCommitment,
    /// Disclose a predicate proof rather than raw data.
    PredicateProof,
    /// Disclose that data exists but redact the content.
    RedactedPresence,
    /// Withhold the field completely.
    Withheld,
}

impl ProtocolProfile {
    /// Returns true when a profile requires signed consequential invocations.
    #[must_use]
    pub const fn requires_signed_consequential_invocations(self) -> bool {
        matches!(self, Self::Federated | Self::Sovereign)
    }

    /// Returns true when the profile forbids state-changing early data.
    #[must_use]
    pub const fn forbids_state_changing_early_data(self) -> bool {
        matches!(self, Self::Sovereign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovereign_profile_is_strict() {
        let profile = ProtocolProfile::Sovereign;
        assert!(profile.requires_signed_consequential_invocations());
        assert!(profile.forbids_state_changing_early_data());
    }
}
