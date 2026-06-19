#![no_std]
#![doc = "Core identifiers and validation primitives for BCX."]

#[cfg(test)]
extern crate std;

mod error;
mod ids;

pub use error::ValidationError;
pub use ids::{
    CapabilityRef, CheckpointId, Digest, EventId, NativeBindingId, Nonce, OperationSequence,
    PolicyEpoch, PolicyId, ProfileId, ProofSuiteId, RealmId, StatementId, SubjectId,
};

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

    #[test]
    fn statement_and_checkpoint_ids_require_digest_length() {
        assert_eq!(StatementId::new(&[]), Err(ValidationError::Empty));
        assert_eq!(
            StatementId::new(&[1; Digest::LEN - 1]),
            Err(ValidationError::Malformed)
        );
        assert_eq!(
            CheckpointId::new(&[1; Digest::LEN + 1]),
            Err(ValidationError::TooLarge)
        );

        let statement = StatementId::new(&[1; Digest::LEN]);
        assert_eq!(statement.map(|id| id.len()), Ok(Digest::LEN));
    }

    #[test]
    fn public_identifier_constructors_reject_zero_values() {
        assert_eq!(
            StatementId::new(&[0; Digest::LEN]),
            Err(ValidationError::ZeroValue)
        );
        assert_eq!(SubjectId::new(&[0]), Err(ValidationError::ZeroValue));
        assert_eq!(RealmId::new(&[0]), Err(ValidationError::ZeroValue));
        assert_eq!(ProfileId::new(&[0]), Err(ValidationError::ZeroValue));
        assert_eq!(ProofSuiteId::new(&[0]), Err(ValidationError::ZeroValue));
        assert_eq!(PolicyId::new(&[0]), Err(ValidationError::ZeroValue));
        assert_eq!(
            CheckpointId::new(&[0; Digest::LEN]),
            Err(ValidationError::ZeroValue)
        );
        assert_eq!(NativeBindingId::new(&[0]), Err(ValidationError::ZeroValue));
    }

    #[test]
    fn bounded_identifiers_validate_lengths_and_preserve_bytes() -> Result<(), ValidationError> {
        let subject = SubjectId::new(b"subject:invoice:123")?;
        let realm = RealmId::new(b"realm:valkyoth")?;
        let profile = ProfileId::new(b"bcx-core")?;
        let proof_suite = ProofSuiteId::new(b"ed25519")?;
        let policy = PolicyId::new(b"strict")?;
        let native = NativeBindingId::new(b"fluxheim/request")?;

        assert_eq!(subject.as_bytes(), b"subject:invoice:123");
        assert_eq!(realm.as_bytes(), b"realm:valkyoth");
        assert_eq!(profile.as_bytes(), b"bcx-core");
        assert_eq!(proof_suite.as_bytes(), b"ed25519");
        assert_eq!(policy.as_bytes(), b"strict");
        assert_eq!(native.as_bytes(), b"fluxheim/request");
        assert_eq!(
            SubjectId::new(&[1; SubjectId::MAX_LEN + 1]),
            Err(ValidationError::TooLarge)
        );
        Ok(())
    }
}
