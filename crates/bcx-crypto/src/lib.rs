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
    fn hybrid_signature_split_uses_documented_layout() {
        let signature = [5; SignatureAlgorithm::HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN];
        let split = SignatureAlgorithm::HybridEd25519MlDsa65.split_hybrid(&signature);

        assert_eq!(
            split.map(|(ed25519, ml_dsa)| (ed25519.len(), ml_dsa.len())),
            Some((
                SignatureAlgorithm::ED25519_SIGNATURE_LEN,
                SignatureAlgorithm::ML_DSA_65_SIGNATURE_LEN,
            ))
        );
        assert_eq!(SignatureAlgorithm::Ed25519.split_hybrid(&signature), None);
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

    #[test]
    fn detached_payload_is_bounded_by_wire_limits() -> Result<(), VerificationError> {
        struct AcceptingVerifier;

        impl Verifier for AcceptingVerifier {
            fn verify(
                &self,
                _envelope: &SignatureEnvelope<'_>,
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
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
        let payload = [0; 65];
        let limits =
            WireLimits::new(64, 1, 1, 1).map_err(|_| VerificationError::InvalidSignature)?;
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::Ed25519]);

        assert_eq!(
            envelope.verify_detached_bytes(&AcceptingVerifier, &policy, &payload, limits),
            Err(VerificationError::PayloadTooLarge)
        );
        Ok(())
    }
}
