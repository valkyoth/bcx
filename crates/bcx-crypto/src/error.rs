/// Verification and signature-envelope validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    /// The signature payload was empty.
    EmptySignature,
    /// The signature payload exceeded the active profile bound.
    SignatureTooLarge,
    /// The key identifier was all zeros.
    EmptyKeyId,
    /// The algorithm is not admitted by local policy.
    AlgorithmNotAdmitted,
    /// Signature verification failed.
    InvalidSignature,
}
