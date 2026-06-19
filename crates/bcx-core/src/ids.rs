use crate::ValidationError;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

/// Fixed-width digest used for protocol commitments.
#[derive(Clone, Copy, Eq)]
pub struct Digest([u8; Self::LEN]);

impl Digest {
    /// Digest byte length for the first BCX profile.
    pub const LEN: usize = 32;

    /// Creates a digest from raw bytes.
    #[must_use]
    pub const fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }

    /// Returns the digest as bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.0
    }

    /// Returns true when every byte is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        bool::from(self.0.ct_eq(&[0u8; Self::LEN]))
    }

    /// Compares two digests without data-dependent early exit.
    ///
    /// This is also used by `PartialEq` to avoid byte-by-byte early exit.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        bool::from(self.0.ct_eq(&other.0))
    }
}

impl PartialEq for Digest {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl core::hash::Hash for Digest {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.0, state);
    }
}

impl core::fmt::Debug for Digest {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("Digest(..)")
    }
}

/// Globally unique event identifier within a trust domain.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EventId(Digest);

impl EventId {
    /// Creates an event identifier when the digest is non-zero.
    pub fn new(digest: Digest) -> Result<Self, ValidationError> {
        if digest.is_zero() {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(digest))
        }
    }

    /// Returns the underlying digest commitment.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.0
    }
}

/// Reference to a capability object or capability commitment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CapabilityRef(Digest);

impl CapabilityRef {
    /// Creates a capability reference when the digest is non-zero.
    pub fn new(digest: Digest) -> Result<Self, ValidationError> {
        if digest.is_zero() {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(digest))
        }
    }

    /// Returns the underlying digest commitment.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.0
    }
}

/// Reference to a policy epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PolicyEpoch(Digest);

impl PolicyEpoch {
    /// Creates a policy epoch when the digest is non-zero.
    pub fn new(digest: Digest) -> Result<Self, ValidationError> {
        if digest.is_zero() {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(digest))
        }
    }

    /// Returns the underlying digest commitment.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.0
    }
}

/// Per-issuer operation sequence used for replay detection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OperationSequence(u64);

impl OperationSequence {
    /// Creates a non-zero operation sequence.
    pub const fn new(value: u64) -> Result<Self, ValidationError> {
        if value == 0 {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the raw sequence number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next sequence value.
    pub const fn next(self) -> Result<Self, ValidationError> {
        match self.0.checked_add(1) {
            Some(value) => Ok(Self(value)),
            None => Err(ValidationError::TooLarge),
        }
    }

    /// Returns true when this sequence immediately follows `previous`.
    #[must_use]
    pub const fn immediately_follows(self, previous: Self) -> bool {
        match previous.0.checked_add(1) {
            Some(expected) => self.0 == expected,
            None => false,
        }
    }
}

/// Nonce bytes carried by signed invocations and WHY queries.
#[derive(Clone, Eq)]
pub struct Nonce([u8; Self::LEN]);

impl Nonce {
    /// Nonce byte length for the first BCX profile.
    pub const LEN: usize = 16;

    /// Creates a nonce from non-zero raw bytes.
    pub fn new(bytes: [u8; Self::LEN]) -> Result<Self, ValidationError> {
        if bool::from(bytes.ct_eq(&[0u8; Self::LEN])) {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(bytes))
        }
    }

    /// Returns the nonce as bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.0
    }

    /// Compares two nonces without data-dependent early exit.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        bool::from(self.0.ct_eq(&other.0))
    }
}

impl PartialEq for Nonce {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl core::hash::Hash for Nonce {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.0, state);
    }
}

impl Drop for Nonce {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl core::fmt::Debug for Nonce {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("Nonce(..)")
    }
}
