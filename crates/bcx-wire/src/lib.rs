#![no_std]
#![doc = "Wire versioning and bounded-message primitives for BCX."]

use bcx_core::ValidationError;

/// BCX protocol version negotiated by a transport binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersion {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
}

impl ProtocolVersion {
    /// Current implemented protocol version.
    pub const CURRENT: Self = Self::new(1, 0);

    /// Creates a protocol version.
    #[must_use]
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }
}

/// Conservative limits applied before expensive parsing or verification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireLimits {
    /// Maximum single message length in bytes.
    maximum_message_len: usize,
    /// Maximum parent events in a compact cause capsule.
    maximum_parent_events: usize,
    /// Maximum WHY graph traversal depth.
    maximum_why_depth: usize,
    /// Maximum events returned by one explanation query.
    maximum_explanation_events: usize,
}

impl WireLimits {
    /// Hard upper bound for one canonical message.
    pub const MAXIMUM_MESSAGE_LEN: usize = 16 * 1024 * 1024;
    /// Hard upper bound for compact parent references.
    pub const MAXIMUM_PARENT_EVENTS: usize = 1_024;
    /// Hard upper bound for WHY traversal depth.
    pub const MAXIMUM_WHY_DEPTH: usize = 32;
    /// Hard upper bound for events returned in one explanation.
    pub const MAXIMUM_EXPLANATION_EVENTS: usize = 10_000;

    /// Default limits for the first development profile.
    pub const DEVELOPMENT: Self = Self {
        maximum_message_len: 1_048_576,
        maximum_parent_events: 16,
        maximum_why_depth: 5,
        maximum_explanation_events: 100,
    };

    /// Creates validated wire limits.
    pub const fn new(
        maximum_message_len: usize,
        maximum_parent_events: usize,
        maximum_why_depth: usize,
        maximum_explanation_events: usize,
    ) -> Result<Self, ValidationError> {
        if maximum_message_len == 0
            || maximum_parent_events == 0
            || maximum_why_depth == 0
            || maximum_explanation_events == 0
        {
            return Err(ValidationError::Empty);
        }
        if maximum_message_len > Self::MAXIMUM_MESSAGE_LEN
            || maximum_parent_events > Self::MAXIMUM_PARENT_EVENTS
            || maximum_why_depth > Self::MAXIMUM_WHY_DEPTH
            || maximum_explanation_events > Self::MAXIMUM_EXPLANATION_EVENTS
        {
            return Err(ValidationError::TooLarge);
        }
        Ok(Self {
            maximum_message_len,
            maximum_parent_events,
            maximum_why_depth,
            maximum_explanation_events,
        })
    }

    /// Returns the maximum single message length in bytes.
    #[must_use]
    pub const fn maximum_message_len(self) -> usize {
        self.maximum_message_len
    }

    /// Returns the maximum parent events in a compact cause capsule.
    #[must_use]
    pub const fn maximum_parent_events(self) -> usize {
        self.maximum_parent_events
    }

    /// Returns the maximum WHY graph traversal depth.
    #[must_use]
    pub const fn maximum_why_depth(self) -> usize {
        self.maximum_why_depth
    }

    /// Returns the maximum events returned by one explanation query.
    #[must_use]
    pub const fn maximum_explanation_events(self) -> usize {
        self.maximum_explanation_events
    }
}

/// Fixed header metadata common to profile-carried BCX messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireHeader {
    version: ProtocolVersion,
    payload_len: u32,
}

impl WireHeader {
    /// Creates a validated wire header.
    pub const fn new(
        version: ProtocolVersion,
        payload_len: u32,
        limits: WireLimits,
    ) -> Result<Self, ValidationError> {
        let header = Self {
            version,
            payload_len,
        };
        match header.validate(limits) {
            Ok(()) => Ok(header),
            Err(error) => Err(error),
        }
    }

    /// Validates protocol version and payload length.
    pub const fn validate(&self, limits: WireLimits) -> Result<(), ValidationError> {
        if self.version.major != ProtocolVersion::CURRENT.major {
            return Err(ValidationError::NotPermitted);
        }
        if self.version.minor != ProtocolVersion::CURRENT.minor {
            return Err(ValidationError::NotPermitted);
        }
        if self.payload_len == 0 {
            return Err(ValidationError::Empty);
        }
        if (self.payload_len as usize) > limits.maximum_message_len() {
            return Err(ValidationError::TooLarge);
        }
        Ok(())
    }

    /// Returns the protocol version.
    #[must_use]
    pub const fn version(self) -> ProtocolVersion {
        self.version
    }

    /// Returns the canonical payload length in bytes.
    #[must_use]
    pub const fn payload_len(self) -> u32 {
        self.payload_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_rejects_empty_payload() {
        assert_eq!(
            WireHeader::new(ProtocolVersion::CURRENT, 0, WireLimits::DEVELOPMENT),
            Err(ValidationError::Empty)
        );
    }

    #[test]
    fn header_rejects_future_minor_version() {
        assert_eq!(
            WireHeader::new(ProtocolVersion::new(1, 1), 1, WireLimits::DEVELOPMENT),
            Err(ValidationError::NotPermitted)
        );
    }

    #[test]
    fn limits_reject_unbounded_values() {
        assert_eq!(
            WireLimits::new(usize::MAX, 1, 1, 1),
            Err(ValidationError::TooLarge)
        );
        assert_eq!(WireLimits::new(1, 0, 1, 1), Err(ValidationError::Empty));
    }
}
