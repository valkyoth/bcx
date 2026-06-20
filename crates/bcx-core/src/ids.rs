use crate::ValidationError;
use subtle::{Choice, ConstantTimeEq};
use zeroize::Zeroize;

#[derive(Eq)]
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
        if bytes.len() > MAX {
            return Err(ValidationError::TooLarge);
        }
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(ValidationError::ZeroValue);
        }

        let len = u8::try_from(bytes.len()).map_err(|_| ValidationError::TooLarge)?;
        let mut stored = [0; MAX];
        stored[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { bytes: stored, len })
    }

    fn as_bytes(&self) -> &[u8] {
        self.bytes.split_at(self.len as usize).0
    }

    const fn len(&self) -> usize {
        self.len as usize
    }

    fn ct_eq(&self, other: &Self) -> bool {
        let len_eq: Choice = self.len.ct_eq(&other.len);
        let bytes_eq: Choice = self.bytes.ct_eq(&other.bytes);
        bool::from(len_eq & bytes_eq)
    }
}

impl<const MIN: usize, const MAX: usize> PartialEq for IdentifierBytes<MIN, MAX> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl<const MIN: usize, const MAX: usize> Drop for IdentifierBytes<MIN, MAX> {
    fn drop(&mut self) {
        self.bytes.zeroize();
        self.len.zeroize();
    }
}

macro_rules! define_identifier {
    ($(#[$meta:meta])* $name:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[derive(Eq, PartialEq)]
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
///
/// `Digest` is `Copy` for ergonomic identifier wrappers and therefore cannot
/// zero its backing bytes on drop. Use [`ZeroizedDigest`] at processing
/// boundaries where digest residue must not survive the scope.
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

/// Digest wrapper that clears its backing bytes when dropped.
///
/// Use this for key identifiers, event commitments, or capability references
/// held at a boundary where memory residue is part of the threat model. The
/// base [`Digest`] remains `Copy`; this wrapper intentionally does not.
#[derive(Eq)]
pub struct ZeroizedDigest(Digest);

impl ZeroizedDigest {
    /// Wraps a digest so its local backing bytes are cleared on drop.
    #[must_use]
    pub const fn new(digest: Digest) -> Self {
        Self(digest)
    }

    /// Returns a copy of the wrapped digest.
    ///
    /// The returned [`Digest`] is `Copy` and will not be zeroed on drop. Do not
    /// store the return value beyond its immediate use.
    #[must_use]
    pub const fn digest(&self) -> Digest {
        self.0
    }

    /// Returns the wrapped digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Digest::LEN] {
        self.0.as_bytes()
    }

    /// Compares two wrapped digests without data-dependent early exit.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.0.ct_eq(&other.0)
    }
}

impl From<Digest> for ZeroizedDigest {
    fn from(digest: Digest) -> Self {
        Self::new(digest)
    }
}

impl PartialEq for ZeroizedDigest {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl Drop for ZeroizedDigest {
    fn drop(&mut self) {
        self.0.0.zeroize();
    }
}

impl core::fmt::Debug for ZeroizedDigest {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("ZeroizedDigest(..)")
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
/// It also intentionally does not implement `Hash`; replay caches should use
/// a keyed or constant-time structure instead of exposing nonce bytes to a
/// general-purpose hash table.
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
