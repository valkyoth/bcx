/// Validation failures shared by BCX crates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationError {
    /// A value was empty where the protocol requires at least one byte or item.
    Empty,
    /// A value exceeded its active protocol or profile bound.
    TooLarge,
    /// A zero value was supplied for a field that must be non-zero.
    ZeroValue,
    /// A field was malformed for its declared type.
    Malformed,
    /// A field was valid in shape but not permitted by the active profile.
    NotPermitted,
}
