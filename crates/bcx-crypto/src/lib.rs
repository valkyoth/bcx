#![no_std]
#![doc = "Crypto-agile envelope metadata for BCX."]

mod envelope;
mod error;

pub use envelope::{SignatureAlgorithm, SignatureEnvelope, SignedEnvelope, Verifier};
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
}
