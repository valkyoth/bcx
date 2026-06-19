use crate::VerificationError;
use bcx_core::Digest;

/// Signature algorithms named by BCX metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureAlgorithm {
    /// Ed25519 for compact classical signatures.
    Ed25519,
    /// ML-DSA-65 for post-quantum-ready deployments.
    MlDsa65,
    /// SLH-DSA-SHA2-128s for conservative stateless signatures.
    SlhDsaSha2_128s,
    /// Hybrid Ed25519 plus ML-DSA-65 signature envelope.
    HybridEd25519MlDsa65,
}

impl SignatureAlgorithm {
    /// Returns the exact signature length admitted for this algorithm.
    #[must_use]
    pub const fn expected_signature_len(self) -> usize {
        match self {
            Self::Ed25519 => 64,
            Self::MlDsa65 => 3_293,
            Self::SlhDsaSha2_128s => 7_856,
            Self::HybridEd25519MlDsa65 => 3_357,
        }
    }
}

/// Closed algorithm admission policy for a verification context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AlgorithmPolicy<'a> {
    admitted: &'a [SignatureAlgorithm],
}

impl<'a> AlgorithmPolicy<'a> {
    /// Creates an algorithm admission policy from an explicit allow-list.
    #[must_use]
    pub const fn new(admitted: &'a [SignatureAlgorithm]) -> Self {
        Self { admitted }
    }

    /// Returns true when the algorithm is admitted by this policy.
    #[must_use]
    pub const fn admits(&self, algorithm: SignatureAlgorithm) -> bool {
        let mut index = 0;
        while index < self.admitted.len() {
            if self.admitted[index].eq_const(algorithm) {
                return true;
            }
            index += 1;
        }
        false
    }
}

impl SignatureAlgorithm {
    const fn eq_const(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Ed25519, Self::Ed25519)
                | (Self::MlDsa65, Self::MlDsa65)
                | (Self::SlhDsaSha2_128s, Self::SlhDsaSha2_128s)
                | (Self::HybridEd25519MlDsa65, Self::HybridEd25519MlDsa65)
        )
    }
}

/// Signature metadata over a canonical BCX payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignatureEnvelope<'a> {
    /// Commitment to the signing key or certificate chain.
    pub key_id: Digest,
    /// Signature algorithm identifier.
    pub algorithm: SignatureAlgorithm,
    /// Raw signature bytes.
    pub signature: &'a [u8],
}

impl SignatureEnvelope<'_> {
    /// Validates envelope shape before algorithm dispatch.
    pub const fn validate(&self, maximum_signature_len: usize) -> Result<(), VerificationError> {
        if self.key_id.is_zero() {
            return Err(VerificationError::EmptyKeyId);
        }
        if self.signature.is_empty() {
            return Err(VerificationError::EmptySignature);
        }
        if self.signature.len() > maximum_signature_len {
            return Err(VerificationError::SignatureTooLarge);
        }
        if self.signature.len() != self.algorithm.expected_signature_len() {
            return Err(VerificationError::InvalidSignature);
        }
        Ok(())
    }
}

/// Payload paired with a signature envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedEnvelope<'a, T> {
    /// Canonical payload value.
    pub payload: T,
    /// Signature envelope over the canonical payload bytes.
    pub signature: SignatureEnvelope<'a>,
}

impl<'a, T> SignedEnvelope<'a, T> {
    /// Verifies this envelope against caller-provided canonical bytes.
    pub fn verify_bytes<V: Verifier>(
        &self,
        verifier: &V,
        algorithm_policy: &AlgorithmPolicy<'_>,
        canonical_payload: &[u8],
        maximum_signature_len: usize,
    ) -> Result<(), VerificationError> {
        if !algorithm_policy.admits(self.signature.algorithm) {
            return Err(VerificationError::AlgorithmNotAdmitted);
        }
        self.signature.validate(maximum_signature_len)?;
        verifier.verify(&self.signature, canonical_payload)
    }
}

/// Signature verification backend boundary.
pub trait Verifier {
    /// Verifies one signature envelope over canonical payload bytes.
    fn verify(
        &self,
        envelope: &SignatureEnvelope<'_>,
        canonical_payload: &[u8],
    ) -> Result<(), VerificationError>;
}
