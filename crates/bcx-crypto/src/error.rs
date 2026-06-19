/// Verification and signature-envelope validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    /// The signature payload was empty.
    EmptySignature,
    /// The signature payload exceeded the active profile bound.
    SignatureTooLarge,
    /// The detached canonical payload exceeded the active profile bound.
    PayloadTooLarge,
    /// The key identifier was all zeros.
    EmptyKeyId,
    /// The algorithm policy was empty without using an explicit deny-all policy.
    EmptyAlgorithmPolicy,
    /// The algorithm is not admitted by local policy.
    AlgorithmNotAdmitted,
    /// Signature verification failed.
    InvalidSignature,
}
