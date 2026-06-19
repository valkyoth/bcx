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
        CapabilityRef, Digest, EventId, Nonce, OperationSequence, PolicyEpoch, ValidationError,
    };
    pub use bcx_crypto::{SignatureAlgorithm, SignatureEnvelope, VerificationError, Verifier};
    pub use bcx_model::{
        AdmissionResult, AssuranceLevel, CauseKind, EffectResult, OperationAction,
        RelationshipKind, TruthStatus,
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
}
