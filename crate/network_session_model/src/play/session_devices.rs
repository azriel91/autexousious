use std::str::FromStr;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::play::{NetworkSessionModelError, SessionDevice};

/// Devices in the network session.
///
/// Newtype for `Vec<SessionDevice>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct SessionDevices(pub Vec<SessionDevice>);

impl FromStr for SessionDevices {
    type Err = NetworkSessionModelError;

    fn from_str(session_devices_str: &str) -> Result<Self, NetworkSessionModelError> {
        session_devices_str
            .split_whitespace()
            .try_fold(
                SessionDevices::default(),
                |mut session_devices, session_device_str| {
                    let session_device = SessionDevice::from_str(session_device_str)?;
                    session_devices.push(session_device);

                    Ok(session_devices)
                },
            )
            .map_err(|_: NetworkSessionModelError| {
                NetworkSessionModelError::SessionDevicesParseError
            })
    }
}
