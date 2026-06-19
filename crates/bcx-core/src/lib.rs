#![no_std]
#![doc = "Core identifiers and validation primitives for BCX."]

mod error;
mod ids;

pub use error::ValidationError;
pub use ids::{CapabilityRef, Digest, EventId, Nonce, OperationSequence, PolicyEpoch};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_digest_is_detected() {
        let digest = Digest::new([0; Digest::LEN]);
        assert!(digest.is_zero());
    }

    #[test]
    fn operation_sequence_rejects_zero() {
        assert_eq!(OperationSequence::new(0), Err(ValidationError::ZeroValue));
        assert_eq!(OperationSequence::new(7).map(OperationSequence::get), Ok(7));
    }
}
