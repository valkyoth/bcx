use crate::VerificationError;
use bcx_core::Digest;
use bcx_wire::WireLimits;
use core::fmt;

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
    ///
    /// Canonical layout is `[ed25519: 64 bytes][ml-dsa-65: 3293 bytes]`.
    /// Verifiers must verify both components before returning `Ok`.
    HybridEd25519MlDsa65,
}

impl SignatureAlgorithm {
    /// Ed25519 signature byte length.
    pub const ED25519_SIGNATURE_LEN: usize = 64;
    /// ML-DSA-65 signature byte length.
    pub const ML_DSA_65_SIGNATURE_LEN: usize = 3_293;
    /// SLH-DSA-SHA2-128s signature byte length.
    pub const SLH_DSA_SHA2_128S_SIGNATURE_LEN: usize = 7_856;
    /// Hybrid Ed25519 plus ML-DSA-65 signature byte length.
    pub const HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN: usize =
        Self::ED25519_SIGNATURE_LEN + Self::ML_DSA_65_SIGNATURE_LEN;

    /// Returns the exact signature length admitted for this algorithm.
    #[must_use]
    pub const fn expected_signature_len(self) -> usize {
        match self {
            Self::Ed25519 => Self::ED25519_SIGNATURE_LEN,
            Self::MlDsa65 => Self::ML_DSA_65_SIGNATURE_LEN,
            Self::SlhDsaSha2_128s => Self::SLH_DSA_SHA2_128S_SIGNATURE_LEN,
            Self::HybridEd25519MlDsa65 => Self::HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN,
        }
    }

    /// Splits a hybrid signature into Ed25519 and ML-DSA-65 components.
    ///
    /// Layout: `[ed25519: 64 bytes][ml-dsa-65: 3293 bytes]`. Verifiers for
    /// `HybridEd25519MlDsa65` must verify both returned components.
    #[must_use]
    pub fn split_hybrid(self, signature: &[u8]) -> Option<(&[u8], &[u8])> {
        match self {
            Self::HybridEd25519MlDsa65
                if signature.len() == Self::HYBRID_ED25519_ML_DSA_65_SIGNATURE_LEN =>
            {
                Some(signature.split_at(Self::ED25519_SIGNATURE_LEN))
            }
            Self::Ed25519 | Self::MlDsa65 | Self::SlhDsaSha2_128s | Self::HybridEd25519MlDsa65 => {
                None
            }
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
    ///
    /// Admitting several algorithms lets the sender choose any admitted
    /// algorithm, including the weakest one. High-assurance deployments should
    /// admit exactly one algorithm for an operation class unless downgrade
    /// behavior is explicitly part of the profile security contract.
    pub const fn new(admitted: &'a [SignatureAlgorithm]) -> Result<Self, VerificationError> {
        if admitted.is_empty() {
            Err(VerificationError::EmptyAlgorithmPolicy)
        } else {
            Ok(Self { admitted })
        }
    }

    /// Creates an explicit deny-all algorithm policy.
    #[must_use]
    pub const fn deny_all() -> Self {
        Self { admitted: &[] }
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

/// Single-algorithm admission policy for high-assurance verification contexts.
///
/// This avoids sender-choice downgrade behavior entirely: exactly one
/// algorithm is admitted for the operation class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactAlgorithmPolicy {
    admitted: SignatureAlgorithm,
}

impl ExactAlgorithmPolicy {
    /// Creates a policy that admits exactly one signature algorithm.
    #[must_use]
    pub const fn new(admitted: SignatureAlgorithm) -> Self {
        Self { admitted }
    }

    /// Returns the only admitted algorithm.
    #[must_use]
    pub const fn algorithm(self) -> SignatureAlgorithm {
        self.admitted
    }

    /// Returns true when the algorithm is the exact admitted algorithm.
    #[must_use]
    pub const fn admits(self, algorithm: SignatureAlgorithm) -> bool {
        self.admitted.eq_const(algorithm)
    }
}

impl SignatureAlgorithm {
    const fn eq_const(self, other: Self) -> bool {
        match (self, other) {
            (Self::Ed25519, Self::Ed25519) => true,
            (Self::Ed25519, Self::MlDsa65) => false,
            (Self::Ed25519, Self::SlhDsaSha2_128s) => false,
            (Self::Ed25519, Self::HybridEd25519MlDsa65) => false,
            (Self::MlDsa65, Self::Ed25519) => false,
            (Self::MlDsa65, Self::MlDsa65) => true,
            (Self::MlDsa65, Self::SlhDsaSha2_128s) => false,
            (Self::MlDsa65, Self::HybridEd25519MlDsa65) => false,
            (Self::SlhDsaSha2_128s, Self::Ed25519) => false,
            (Self::SlhDsaSha2_128s, Self::MlDsa65) => false,
            (Self::SlhDsaSha2_128s, Self::SlhDsaSha2_128s) => true,
            (Self::SlhDsaSha2_128s, Self::HybridEd25519MlDsa65) => false,
            (Self::HybridEd25519MlDsa65, Self::Ed25519) => false,
            (Self::HybridEd25519MlDsa65, Self::MlDsa65) => false,
            (Self::HybridEd25519MlDsa65, Self::SlhDsaSha2_128s) => false,
            (Self::HybridEd25519MlDsa65, Self::HybridEd25519MlDsa65) => true,
        }
    }
}

/// Signature metadata over a canonical BCX payload.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SignatureEnvelope<'a> {
    key_id: Digest,
    algorithm: SignatureAlgorithm,
    signature: &'a [u8],
}

impl<'a> SignatureEnvelope<'a> {
    /// Creates a validated signature envelope.
    pub fn new(
        key_id: Digest,
        algorithm: SignatureAlgorithm,
        signature: &'a [u8],
        limits: WireLimits,
    ) -> Result<Self, VerificationError> {
        let envelope = Self {
            key_id,
            algorithm,
            signature,
        };
        match envelope.validate(limits) {
            Ok(()) => Ok(envelope),
            Err(error) => Err(error),
        }
    }

    /// Validates envelope shape before algorithm dispatch.
    pub(crate) fn validate(&self, limits: WireLimits) -> Result<(), VerificationError> {
        if self.key_id.is_zero() {
            return Err(VerificationError::EmptyKeyId);
        }
        if self.signature.is_empty() {
            return Err(VerificationError::EmptySignature);
        }
        if self.signature.len() > limits.maximum_message_len() {
            return Err(VerificationError::SignatureTooLarge);
        }
        if self.signature.len() != self.algorithm.expected_signature_len() {
            return Err(VerificationError::InvalidSignature);
        }
        Ok(())
    }

    /// Returns the signing key or certificate-chain commitment.
    #[must_use]
    pub const fn key_id(&self) -> Digest {
        self.key_id
    }

    /// Returns the signature algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }

    /// Returns raw signature bytes.
    #[must_use]
    pub const fn signature(&self) -> &'a [u8] {
        self.signature
    }
}

impl<'a> fmt::Debug for SignatureEnvelope<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignatureEnvelope")
            .field("key_id", &self.key_id)
            .field("algorithm", &self.algorithm)
            .field(
                "signature",
                &format_args!("[{} bytes]", self.signature.len()),
            )
            .finish()
    }
}

/// Payload paired with a signature envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedEnvelope<'a, T> {
    payload: T,
    signature: SignatureEnvelope<'a>,
}

impl<'a, T> SignedEnvelope<'a, T> {
    /// Creates a signed envelope from a payload and validated signature metadata.
    #[must_use]
    pub const fn new(payload: T, signature: SignatureEnvelope<'a>) -> Self {
        Self { payload, signature }
    }

    /// Verifies a detached canonical byte representation of this envelope.
    ///
    /// The caller must ensure `canonical_payload` is the exact canonical
    /// encoding of `self.payload()`. BCX will replace this detached helper with
    /// typed canonical encoding once `bcx-codec` is introduced.
    pub fn verify_detached_bytes<V: Verifier>(
        &self,
        verifier: &V,
        algorithm_policy: &AlgorithmPolicy<'_>,
        canonical_payload: &[u8],
        limits: WireLimits,
    ) -> Result<(), VerificationError> {
        if !algorithm_policy.admits(self.signature.algorithm) {
            return Err(VerificationError::AlgorithmNotAdmitted);
        }
        self.verify_admitted_detached_bytes(verifier, canonical_payload, limits)
    }

    /// Verifies detached bytes with an exact single-algorithm policy.
    ///
    /// Prefer this helper for high-assurance profiles where sender-choice
    /// algorithm downgrade is not acceptable.
    pub fn verify_detached_bytes_exact<V: Verifier>(
        &self,
        verifier: &V,
        algorithm_policy: ExactAlgorithmPolicy,
        canonical_payload: &[u8],
        limits: WireLimits,
    ) -> Result<(), VerificationError> {
        if !algorithm_policy.admits(self.signature.algorithm) {
            return Err(VerificationError::AlgorithmNotAdmitted);
        }
        self.verify_admitted_detached_bytes(verifier, canonical_payload, limits)
    }

    fn verify_admitted_detached_bytes<V: Verifier>(
        &self,
        verifier: &V,
        canonical_payload: &[u8],
        limits: WireLimits,
    ) -> Result<(), VerificationError> {
        self.signature.validate(limits)?;
        if canonical_payload.len() > limits.maximum_message_len() {
            return Err(VerificationError::PayloadTooLarge);
        }
        match self.signature.algorithm {
            SignatureAlgorithm::HybridEd25519MlDsa65 => verifier
                .verify_hybrid(&self.signature, canonical_payload)
                .map(|_| ()),
            SignatureAlgorithm::Ed25519
            | SignatureAlgorithm::MlDsa65
            | SignatureAlgorithm::SlhDsaSha2_128s => {
                verifier.verify(&self.signature, canonical_payload)
            }
        }
    }

    /// Returns the payload value.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Returns the signature envelope.
    #[must_use]
    pub const fn signature(&self) -> SignatureEnvelope<'a> {
        self.signature
    }
}

/// Proof that both components of a hybrid signature verified.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HybridVerified(());

/// Verification backend boundary for hybrid signature components.
pub trait HybridVerifier {
    /// Verifies the Ed25519 component of a hybrid signature.
    fn verify_ed25519(
        &self,
        ed25519_signature: &[u8],
        canonical_payload: &[u8],
    ) -> Result<(), VerificationError>;

    /// Verifies the ML-DSA-65 component of a hybrid signature.
    fn verify_ml_dsa_65(
        &self,
        ml_dsa_65_signature: &[u8],
        canonical_payload: &[u8],
    ) -> Result<(), VerificationError>;

    /// Verifies both components of a hybrid signature.
    ///
    /// Implementors must run component verification to completion and must not
    /// use intermediate component failures to skip later component work. The
    /// default implementation always invokes both component methods before
    /// combining their results.
    fn verify_hybrid(
        &self,
        envelope: &SignatureEnvelope<'_>,
        canonical_payload: &[u8],
    ) -> Result<HybridVerified, VerificationError> {
        let (ed25519, ml_dsa_65) = envelope
            .algorithm()
            .split_hybrid(envelope.signature())
            .ok_or(VerificationError::InvalidSignature)?;
        let ed25519_ok = self.verify_ed25519(ed25519, canonical_payload).is_ok();
        let ml_dsa_65_ok = self.verify_ml_dsa_65(ml_dsa_65, canonical_payload).is_ok();
        if ed25519_ok & ml_dsa_65_ok {
            Ok(HybridVerified(()))
        } else {
            Err(VerificationError::InvalidSignature)
        }
    }
}

/// Signature verification backend boundary.
pub trait Verifier: HybridVerifier {
    /// Verifies one signature envelope over canonical payload bytes.
    fn verify(
        &self,
        envelope: &SignatureEnvelope<'_>,
        canonical_payload: &[u8],
    ) -> Result<(), VerificationError>;
}
