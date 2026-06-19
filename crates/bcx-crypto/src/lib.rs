#![no_std]
#![doc = "Crypto-agile envelope metadata for BCX."]

mod envelope;
mod error;

pub use envelope::{
    AlgorithmPolicy, SignatureAlgorithm, SignatureEnvelope, SignedEnvelope, Verifier,
};
pub use error::VerificationError;

#[cfg(test)]
mod tests {
    use super::*;
    use bcx_core::Digest;

    #[test]
    fn empty_signature_is_rejected() {
        let envelope = SignatureEnvelope {
            key_id: Digest::new([1; Digest::LEN]),
            algorithm: SignatureAlgorithm::Ed25519,
            signature: &[],
        };

        assert_eq!(
            envelope.validate(256),
            Err(VerificationError::EmptySignature)
        );
    }

    #[test]
    fn wrong_signature_length_is_rejected() {
        let envelope = SignatureEnvelope {
            key_id: Digest::new([1; Digest::LEN]),
            algorithm: SignatureAlgorithm::Ed25519,
            signature: &[7; 63],
        };

        assert_eq!(
            envelope.validate(256),
            Err(VerificationError::InvalidSignature)
        );
    }

    #[test]
    fn policy_rejects_unadmitted_algorithm_before_verifier() {
        struct RejectingVerifier;

        impl Verifier for RejectingVerifier {
            fn verify(
                &self,
                _envelope: &SignatureEnvelope<'_>,
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Err(VerificationError::InvalidSignature)
            }
        }

        let signature = [1; 64];
        let envelope = SignedEnvelope {
            payload: (),
            signature: SignatureEnvelope {
                key_id: Digest::new([1; Digest::LEN]),
                algorithm: SignatureAlgorithm::Ed25519,
                signature: &signature,
            },
        };
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::MlDsa65]);

        assert_eq!(
            envelope.verify_bytes(&RejectingVerifier, &policy, b"payload", 8_000),
            Err(VerificationError::AlgorithmNotAdmitted)
        );
    }
}
