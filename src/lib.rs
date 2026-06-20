#![no_std]
#![doc = include_str!("../README.md")]

//! Public facade for the BCX protocol crates.

pub use bcx_core as core;
pub use bcx_crypto as crypto;
pub use bcx_model as model;
pub use bcx_policy as policy;
pub use bcx_wire as wire;

pub use bcx_wire::ProtocolVersion;

/// Items commonly needed by BCX integrators.
pub mod prelude {
    pub use bcx_core::{
        CapabilityRef, CheckpointId, Digest, EventId, NativeBindingId, Nonce, OperationSequence,
        PolicyEpoch, PolicyId, ProfileId, ProofSuiteId, RealmId, StatementId, SubjectId,
        ValidationError, ZeroizedDigest,
    };
    pub use bcx_crypto::{
        AlgorithmPolicy, ExactAlgorithmPolicy, SignatureAlgorithm, SignatureEnvelope,
        VerificationError, Verifier,
    };
    pub use bcx_model::{
        Admission, AdmissionResult, AssuranceLevel, CauseKind, Checkpoint, Contradiction,
        Delegation, Effect, EffectResult, Intent, OperationAction, RelationshipKind, Revocation,
        StatementKind, TruthStatus,
    };
    pub use bcx_policy::{DisclosureLevel, ProofLevel, ProtocolProfile};
    pub use bcx_wire::{ProtocolVersion, WireHeader, WireLimits};
}

/// Returns the current BCX wire version implemented by this crate.
#[must_use]
pub const fn protocol_version() -> ProtocolVersion {
    ProtocolVersion::CURRENT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_reports_current_protocol_version() {
        assert_eq!(protocol_version(), ProtocolVersion::new(1, 0));
    }

    #[test]
    fn prelude_exports_zeroized_digest() {
        use crate::prelude::*;

        let digest = ZeroizedDigest::new(Digest::new([1; Digest::LEN]));

        assert_eq!(digest.as_bytes(), &[1; Digest::LEN]);
    }
}
