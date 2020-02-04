use std::fmt;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{Axis, ControllerId};

/// Axis control for a player.
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct PlayerAxisControl {
    /// Control ID of the player.
    pub player: ControllerId,
    /// Game coordinate axis that this controls.
    pub axis: Axis,
}

impl fmt::Display for PlayerAxisControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {} {} axis", self.player, self.axis)
    }
}
