use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Errors when using `network_session_model` types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NetworkSessionModelError {
    /// Failed to parse a `SessionDevice` from string.
    SessionDeviceParseError,
    /// Failed to parse `SessionDevices` from string.
    SessionDevicesParseError,
}

impl Display for NetworkSessionModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionDeviceParseError => write!(
                f,
                "Session device must be in the form:\n\
                \n\
                <device_id>:<device_name>::<controller_id>:<controller_name>[::..]\n\
                \n\
                Example: `0:az_computer::0:azriel::1:friend_a`\n"
            ),
            Self::SessionDevicesParseError => write!(
                f,
                "Session device must be space separated in the form:\n\
                \n\
                <session_device_0> <session_device_1>\n\
                \n\
                Example: `0:az_computer::0:azriel::1:friend_a 1:by_computer::0:byron::1:friend_b`\n"
            ),
        }
    }
}

impl Error for NetworkSessionModelError {}
