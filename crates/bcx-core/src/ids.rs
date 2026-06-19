use crate::ValidationError;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

#[derive(Clone, Copy, Eq)]
struct IdentifierBytes<const MIN: usize, const MAX: usize> {
    bytes: [u8; MAX],
    len: u8,
}

impl<const MIN: usize, const MAX: usize> IdentifierBytes<MIN, MAX> {
    fn new(bytes: &[u8]) -> Result<Self, ValidationError> {
        if bytes.is_empty() {
            return Err(ValidationError::Empty);
        }
        if bytes.len() < MIN {
            return Err(ValidationError::Malformed);
        }
        if bytes.len() > MAX || bytes.len() > u8::MAX as usize {
            return Err(ValidationError::TooLarge);
        }
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(ValidationError::ZeroValue);
        }

        let mut stored = [0; MAX];
        stored[..bytes.len()].copy_from_slice(bytes);
        Ok(Self {
            bytes: stored,
            len: bytes.len() as u8,
        })
    }

    fn as_bytes(&self) -> &[u8] {
        self.bytes.split_at(self.len as usize).0
    }

    const fn len(&self) -> usize {
        self.len as usize
    }

    fn ct_eq(&self, other: &Self) -> bool {
        self.len == other.len && bool::from(self.bytes.ct_eq(&other.bytes))
    }
}

impl<const MIN: usize, const MAX: usize> PartialEq for IdentifierBytes<MIN, MAX> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl<const MIN: usize, const MAX: usize> core::hash::Hash for IdentifierBytes<MIN, MAX> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self.as_bytes(), state);
    }
}

macro_rules! define_identifier {
    ($(#[$meta:meta])* $name:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        pub struct $name(IdentifierBytes<{ $min }, { $max }>);

        impl $name {
            /// Minimum accepted identifier length in bytes.
            pub const MIN_LEN: usize = $min;
            /// Maximum accepted identifier length in bytes.
            pub const MAX_LEN: usize = $max;

            /// Creates a validated identifier from canonical bytes.
            pub fn new(bytes: &[u8]) -> Result<Self, ValidationError> {
                IdentifierBytes::new(bytes).map(Self)
            }

            /// Returns the canonical identifier bytes.
            #[must_use]
            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes()
            }

            /// Returns the canonical identifier length in bytes.
            #[must_use]
            pub const fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns false because validated BCX identifiers cannot be empty.
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                false
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(concat!(stringify!($name), "(..)"))
            }
        }
    };
}

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

define_identifier!(
    /// Statement identifier for BCX causal statements.
    StatementId,
    Digest::LEN,
    Digest::LEN
);

define_identifier!(
    /// Subject identifier for the thing a statement describes or affects.
    SubjectId,
    1,
    64
);

define_identifier!(
    /// Realm identifier for an authority, namespace, tenant, or trust domain.
    RealmId,
    1,
    64
);

define_identifier!(
    /// Profile identifier for a BCX profile or native binding family.
    ProfileId,
    1,
    32
);

define_identifier!(
    /// Proof-suite identifier for signature or verification policy families.
    ProofSuiteId,
    1,
    32
);

define_identifier!(
    /// Policy identifier for disclosure, replay, settlement, or admission rules.
    PolicyId,
    1,
    32
);

define_identifier!(
    /// Checkpoint identifier for committed state, graph, or settlement checkpoints.
    CheckpointId,
    Digest::LEN,
    Digest::LEN
);

define_identifier!(
    /// Native binding identifier for host-system specific anchors.
    NativeBindingId,
    1,
    64
);

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
///
/// `Nonce` intentionally does not implement `Clone` or `Copy`; duplicating
/// nonce bytes increases the number of plaintext copies that must be cleared.
#[derive(Eq)]
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
    /// Hashes nonce bytes for non-adversarial collection use.
    ///
    /// This hash operation is not constant-time. Replay caches for
    /// high-assurance deployments should use a keyed structure that accounts
    /// for timing and bucket-collision behavior instead of treating `Hash` as
    /// a security boundary.
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
