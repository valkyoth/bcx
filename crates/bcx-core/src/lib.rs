#![no_std]
#![doc = "Core identifiers and validation primitives for BCX."]

#[cfg(test)]
extern crate std;

mod error;
mod ids;

pub use error::ValidationError;
pub use ids::{CapabilityRef, Digest, EventId, Nonce, OperationSequence, PolicyEpoch};

#[cfg(test)]
mod tests {
    use super::*;
    use std::format;

    #[test]
    fn zero_digest_is_detected() {
        let digest = Digest::new([0; Digest::LEN]);
        assert!(digest.is_zero());
    }

    #[test]
    fn digest_constant_shape_equality_matches_structural_equality() {
        let left = Digest::new([7; Digest::LEN]);
        let same = Digest::new([7; Digest::LEN]);
        let different = Digest::new([8; Digest::LEN]);

        assert!(left.ct_eq(&same));
        assert!(!left.ct_eq(&different));
    }

    #[test]
    fn nonce_debug_is_redacted() {
        let nonce = Nonce::new([3; Nonce::LEN]);

        assert_eq!(format!("{nonce:?}"), "Nonce(..)");
    }

    #[test]
    fn operation_sequence_rejects_zero() {
        assert_eq!(OperationSequence::new(0), Err(ValidationError::ZeroValue));
        assert_eq!(OperationSequence::new(7).map(OperationSequence::get), Ok(7));
    }
}
