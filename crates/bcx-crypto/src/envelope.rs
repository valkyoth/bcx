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
        canonical_payload: &[u8],
    ) -> Result<(), VerificationError> {
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
