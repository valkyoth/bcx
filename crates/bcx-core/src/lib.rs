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
    use std::{format, string::String};

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

        assert_eq!(
            nonce.map(|value| format!("{value:?}")),
            Ok(String::from("Nonce(..)"))
        );
    }

    #[test]
    fn nonce_rejects_zero_and_compares_constant_shape() {
        let left = Nonce::new([9; Nonce::LEN]);
        let same = Nonce::new([9; Nonce::LEN]);
        let different = Nonce::new([8; Nonce::LEN]);

        assert_eq!(Nonce::new([0; Nonce::LEN]), Err(ValidationError::ZeroValue));
        assert!(matches!((&left, &same), (Ok(a), Ok(b)) if a.ct_eq(b)));
        assert!(matches!((&left, &different), (Ok(a), Ok(b)) if !a.ct_eq(b)));
    }

    #[test]
    fn operation_sequence_rejects_zero() {
        assert_eq!(OperationSequence::new(0), Err(ValidationError::ZeroValue));
        assert_eq!(OperationSequence::new(7).map(OperationSequence::get), Ok(7));
    }

    #[test]
    fn operation_sequence_checks_monotonic_successor() {
        let previous = OperationSequence::new(7);
        let next = previous.and_then(OperationSequence::next);

        assert_eq!(next.map(OperationSequence::get), Ok(8));
        assert!(matches!(
            (OperationSequence::new(8), OperationSequence::new(7)),
            (Ok(current), Ok(previous)) if current.immediately_follows(previous)
        ));
        assert_eq!(
            OperationSequence::new(u64::MAX).and_then(OperationSequence::next),
            Err(ValidationError::TooLarge)
        );
    }
}
