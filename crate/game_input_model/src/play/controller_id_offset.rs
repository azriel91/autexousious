use std::str::FromStr;

use derive_deref::{Deref, DerefMut};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ControllerId;

/// Offset for controller IDs calculated from `PlayerInputConfigs`.
#[derive(
    Clone, Copy, Debug, Default, Deref, DerefMut, Deserialize, PartialEq, Eq, Serialize, new,
)]
pub struct ControllerIdOffset(pub ControllerId);

impl FromStr for ControllerIdOffset {
    type Err = <ControllerId as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<ControllerId>().map(ControllerIdOffset::new)
    }
}
