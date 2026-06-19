use crate::ValidationError;

/// Fixed-width digest used for protocol commitments.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    pub const fn is_zero(&self) -> bool {
        let mut index = 0;
        while index < Self::LEN {
            if self.0[index] != 0 {
                return false;
            }
            index += 1;
        }
        true
    }
}

impl core::fmt::Debug for Digest {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("Digest(..)")
    }
}

/// Globally unique event identifier within a trust domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventId(Digest);

impl EventId {
    /// Creates an event identifier when the digest is non-zero.
    pub const fn new(digest: Digest) -> Result<Self, ValidationError> {
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
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CapabilityRef(Digest);

impl CapabilityRef {
    /// Creates a capability reference when the digest is non-zero.
    pub const fn new(digest: Digest) -> Result<Self, ValidationError> {
        if digest.is_zero() {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(digest))
        }
    }
}

/// Reference to a policy epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PolicyEpoch(Digest);

impl PolicyEpoch {
    /// Creates a policy epoch when the digest is non-zero.
    pub const fn new(digest: Digest) -> Result<Self, ValidationError> {
        if digest.is_zero() {
            Err(ValidationError::ZeroValue)
        } else {
            Ok(Self(digest))
        }
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
}

/// Nonce bytes carried by signed invocations and WHY queries.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nonce([u8; Self::LEN]);

impl Nonce {
    /// Nonce byte length for the first BCX profile.
    pub const LEN: usize = 16;

    /// Creates a nonce from raw bytes.
    #[must_use]
    pub const fn new(bytes: [u8; Self::LEN]) -> Self {
        Self(bytes)
    }

    /// Returns the nonce as bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Self::LEN] {
        &self.0
    }
}
