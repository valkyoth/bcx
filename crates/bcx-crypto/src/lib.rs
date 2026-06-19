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
    use bcx_wire::WireLimits;

    #[test]
    fn empty_signature_is_rejected() {
        assert_eq!(
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::Ed25519,
                &[],
                WireLimits::DEVELOPMENT,
            ),
            Err(VerificationError::EmptySignature)
        );
    }

    #[test]
    fn wrong_signature_length_is_rejected() {
        assert_eq!(
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::Ed25519,
                &[7; 63],
                WireLimits::DEVELOPMENT,
            ),
            Err(VerificationError::InvalidSignature)
        );
    }

    #[test]
    fn policy_rejects_unadmitted_algorithm_before_verifier() -> Result<(), VerificationError> {
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
        let envelope = SignedEnvelope::new(
            (),
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::Ed25519,
                &signature,
                WireLimits::DEVELOPMENT,
            )?,
        );
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::MlDsa65]);

        assert_eq!(
            envelope.verify_detached_bytes(
                &RejectingVerifier,
                &policy,
                b"payload",
                WireLimits::DEVELOPMENT,
            ),
            Err(VerificationError::AlgorithmNotAdmitted)
        );
        Ok(())
    }
}
