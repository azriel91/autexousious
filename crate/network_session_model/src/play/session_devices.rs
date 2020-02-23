use std::str::FromStr;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::play::{SessionDevice, SessionDeviceName, SessionDevicesParseError};

/// Devices in the network session.
///
/// Newtype for `Vec<SessionDevice>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct SessionDevices(pub Vec<SessionDevice>);

impl FromStr for SessionDevices {
    type Err = SessionDevicesParseError;

    fn from_str(session_devices_str: &str) -> Result<Self, SessionDevicesParseError> {
        session_devices_str.split_whitespace().try_fold(
            SessionDevices::default(),
            |mut session_devices, session_device_str| {
                let mut session_device_str_split = session_device_str.split(':');
                let session_device_id = session_device_str_split.next().map(FromStr::from_str);
                let session_device_name = session_device_str_split
                    .next()
                    .map(String::from)
                    .map(SessionDeviceName::from);

                if let (Some(Ok(session_device_id)), Some(session_device_name)) =
                    (session_device_id, session_device_name)
                {
                    let session_device = SessionDevice::new(session_device_id, session_device_name);
                    session_devices.push(session_device);

                    Ok(session_devices)
                } else {
                    Err(SessionDevicesParseError)
                }
            },
        )
    }
}
