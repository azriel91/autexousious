use std::{num::ParseIntError, str::FromStr};

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Server generated ID for a session device (`u64` newtype).
#[derive(
    Clone, Copy, Debug, Default, Deref, DerefMut, Deserialize, Eq, Hash, PartialEq, Serialize, new,
)]
pub struct SessionDeviceId(pub u64);

impl FromStr for SessionDeviceId {
    type Err = ParseIntError;

    fn from_str(session_device_id_str: &str) -> Result<Self, ParseIntError> {
        session_device_id_str.parse::<u64>().map(SessionDeviceId)
    }
}
