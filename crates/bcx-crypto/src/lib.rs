#![no_std]
#![doc = "Crypto-agile envelope metadata for BCX."]

mod envelope;
mod error;

pub use envelope::{
    AlgorithmPolicy, ExactAlgorithmPolicy, HybridVerified, HybridVerifier, SignatureAlgorithm,
    SignatureEnvelope, SignedEnvelope, Verifier,
};
pub use error::VerificationError;

#[cfg(test)]
mod tests {
    use super::*;
    use bcx_core::Digest;
    use bcx_wire::WireLimits;
    use core::cell::Cell;

    #[test]
    fn empty_signature_is_rejected() {
        assert_eq!(
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::Ed25519,
                &[],
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
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
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
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
    fn empty_algorithm_policy_requires_explicit_deny_all() {
        assert_eq!(
            AlgorithmPolicy::new(&[]),
            Err(VerificationError::EmptyAlgorithmPolicy)
        );
        assert!(!AlgorithmPolicy::deny_all().admits(SignatureAlgorithm::Ed25519));
    }

    #[test]
    fn exact_algorithm_policy_admits_only_one_algorithm() {
        let policy = ExactAlgorithmPolicy::new(SignatureAlgorithm::HybridEd25519MlDsa65);

        assert_eq!(policy.algorithm(), SignatureAlgorithm::HybridEd25519MlDsa65);
        assert!(policy.admits(SignatureAlgorithm::HybridEd25519MlDsa65));
        assert!(!policy.admits(SignatureAlgorithm::Ed25519));
    }

    #[test]
    fn policy_rejects_unadmitted_algorithm_before_verifier() -> Result<(), VerificationError> {
        struct RejectingVerifier;

        impl HybridVerifier for RejectingVerifier {
            fn verify_ed25519(
                &self,
                _ed25519_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Err(VerificationError::InvalidSignature)
            }

            fn verify_ml_dsa_65(
                &self,
                _ml_dsa_65_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Err(VerificationError::InvalidSignature)
            }
        }

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
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            )?,
        );
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::MlDsa65])?;

        assert_eq!(
            envelope.verify_detached_bytes(
                &RejectingVerifier,
                &policy,
                b"payload",
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(VerificationError::AlgorithmNotAdmitted)
        );
        Ok(())
    }

    #[test]
    fn detached_payload_is_bounded_by_wire_limits() -> Result<(), VerificationError> {
        struct AcceptingVerifier;

        impl HybridVerifier for AcceptingVerifier {
            fn verify_ed25519(
                &self,
                _ed25519_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }

            fn verify_ml_dsa_65(
                &self,
                _ml_dsa_65_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }
        }

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
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            )?,
        );
        let payload = [0; 65];
        let limits =
            WireLimits::new(64, 1, 1, 1).map_err(|_| VerificationError::InvalidSignature)?;
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::Ed25519])?;

        assert_eq!(
            envelope.verify_detached_bytes(&AcceptingVerifier, &policy, &payload, limits),
            Err(VerificationError::PayloadTooLarge)
        );
        Ok(())
    }

    #[test]
    fn exact_policy_rejects_algorithm_downgrade() -> Result<(), VerificationError> {
        struct AcceptingVerifier;

        impl HybridVerifier for AcceptingVerifier {
            fn verify_ed25519(
                &self,
                _ed25519_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }

            fn verify_ml_dsa_65(
                &self,
                _ml_dsa_65_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }
        }

        impl Verifier for AcceptingVerifier {
            fn verify(
                &self,
                _envelope: &SignatureEnvelope<'_>,
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }
        }

        let signature = [1; SignatureAlgorithm::ED25519_SIGNATURE_LEN];
        let envelope = SignedEnvelope::new(
            (),
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::Ed25519,
                &signature,
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            )?,
        );
        let policy = ExactAlgorithmPolicy::new(SignatureAlgorithm::HybridEd25519MlDsa65);

        assert_eq!(
            envelope.verify_detached_bytes_exact(
                &AcceptingVerifier,
                policy,
                b"payload",
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(VerificationError::AlgorithmNotAdmitted)
        );
        Ok(())
    }

    #[test]
    fn signature_envelope_debug_redacts_signature_bytes() -> Result<(), VerificationError> {
        extern crate std;
        use std::{format, string::String};

        let signature = [7; SignatureAlgorithm::ED25519_SIGNATURE_LEN];
        let envelope = SignatureEnvelope::new(
            Digest::new([1; Digest::LEN]),
            SignatureAlgorithm::Ed25519,
            &signature,
            WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
        )?;

        assert_eq!(
            format!("{envelope:?}"),
            String::from(
                "SignatureEnvelope { key_id: Digest(..), algorithm: Ed25519, signature: [64 bytes] }"
            )
        );
        Ok(())
    }

    #[test]
    fn hybrid_verification_requires_both_components() -> Result<(), VerificationError> {
        struct PartialHybridVerifier;

        impl HybridVerifier for PartialHybridVerifier {
            fn verify_ed25519(
                &self,
                ed25519_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                if ed25519_signature.len() == SignatureAlgorithm::ED25519_SIGNATURE_LEN {
                    Ok(())
                } else {
                    Err(VerificationError::InvalidSignature)
                }
            }

            fn verify_ml_dsa_65(
                &self,
                _ml_dsa_65_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Err(VerificationError::InvalidSignature)
            }
        }

        impl Verifier for PartialHybridVerifier {
            fn verify(
                &self,
                _envelope: &SignatureEnvelope<'_>,
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Ok(())
            }
        }

        let signature = [9; SignatureAlgorithm::HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN];
        let envelope = SignedEnvelope::new(
            (),
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::HybridEd25519MlDsa65,
                &signature,
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            )?,
        );
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::HybridEd25519MlDsa65])?;

        assert_eq!(
            envelope.verify_detached_bytes(
                &PartialHybridVerifier,
                &policy,
                b"payload",
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(VerificationError::InvalidSignature)
        );
        Ok(())
    }

    #[test]
    fn hybrid_verification_invokes_both_components_on_failure() -> Result<(), VerificationError> {
        struct CountingHybridVerifier {
            ed25519_calls: Cell<usize>,
            ml_dsa_65_calls: Cell<usize>,
        }

        impl HybridVerifier for CountingHybridVerifier {
            fn verify_ed25519(
                &self,
                _ed25519_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                self.ed25519_calls.set(self.ed25519_calls.get() + 1);
                Err(VerificationError::InvalidSignature)
            }

            fn verify_ml_dsa_65(
                &self,
                _ml_dsa_65_signature: &[u8],
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                self.ml_dsa_65_calls.set(self.ml_dsa_65_calls.get() + 1);
                Err(VerificationError::InvalidSignature)
            }
        }

        impl Verifier for CountingHybridVerifier {
            fn verify(
                &self,
                _envelope: &SignatureEnvelope<'_>,
                _canonical_payload: &[u8],
            ) -> Result<(), VerificationError> {
                Err(VerificationError::InvalidSignature)
            }
        }

        let signature = [9; SignatureAlgorithm::HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN];
        let envelope = SignedEnvelope::new(
            (),
            SignatureEnvelope::new(
                Digest::new([1; Digest::LEN]),
                SignatureAlgorithm::HybridEd25519MlDsa65,
                &signature,
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            )?,
        );
        let policy = AlgorithmPolicy::new(&[SignatureAlgorithm::HybridEd25519MlDsa65])?;
        let verifier = CountingHybridVerifier {
            ed25519_calls: Cell::new(0),
            ml_dsa_65_calls: Cell::new(0),
        };

        assert_eq!(
            envelope.verify_detached_bytes(
                &verifier,
                &policy,
                b"payload",
                WireLimits::UNSAFE_DEVELOPMENT_DO_NOT_USE_IN_PRODUCTION,
            ),
            Err(VerificationError::InvalidSignature)
        );
        assert_eq!(verifier.ed25519_calls.get(), 1);
        assert_eq!(verifier.ml_dsa_65_calls.get(), 1);
        Ok(())
    }
}
