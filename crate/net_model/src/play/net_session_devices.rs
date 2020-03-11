use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::play::NetSessionDevice;

/// Devices in the network session (`Vec<NetSessionDevice>` newtype).
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct NetSessionDevices(pub Vec<NetSessionDevice>);
