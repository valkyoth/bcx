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
    pub maximum_message_len: usize,
    /// Maximum parent events in a compact cause capsule.
    pub maximum_parent_events: usize,
    /// Maximum WHY graph traversal depth.
    pub maximum_why_depth: usize,
    /// Maximum events returned by one explanation query.
    pub maximum_explanation_events: usize,
}

impl WireLimits {
    /// Default limits for the first development profile.
    pub const DEVELOPMENT: Self = Self {
        maximum_message_len: 1_048_576,
        maximum_parent_events: 16,
        maximum_why_depth: 5,
        maximum_explanation_events: 100,
    };
}

/// Fixed header metadata common to native and HTTP-carried BCX messages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireHeader {
    /// Protocol version.
    pub version: ProtocolVersion,
    /// Canonical payload length in bytes.
    pub payload_len: usize,
}

impl WireHeader {
    /// Validates protocol version and payload length.
    pub const fn validate(&self, limits: WireLimits) -> Result<(), ValidationError> {
        if self.version.major != ProtocolVersion::CURRENT.major {
            return Err(ValidationError::NotPermitted);
        }
        if self.payload_len == 0 {
            return Err(ValidationError::Empty);
        }
        if self.payload_len > limits.maximum_message_len {
            return Err(ValidationError::TooLarge);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_rejects_empty_payload() {
        let header = WireHeader {
            version: ProtocolVersion::CURRENT,
            payload_len: 0,
        };

        assert_eq!(
            header.validate(WireLimits::DEVELOPMENT),
            Err(ValidationError::Empty)
        );
    }
}
