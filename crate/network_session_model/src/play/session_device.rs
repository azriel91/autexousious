use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::play::{SessionDeviceId, SessionDeviceName};

/// Name and ID of a session device.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct SessionDevice {
    /// Server generated ID of the session device.
    pub id: SessionDeviceId,
    /// Human readable name of the device.
    pub name: SessionDeviceName,
}
