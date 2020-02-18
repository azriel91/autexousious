use std::fmt;

/// Error when failing to parse session device information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SessionDevicesParseError;

impl fmt::Display for SessionDevicesParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse session devices.")
    }
}
