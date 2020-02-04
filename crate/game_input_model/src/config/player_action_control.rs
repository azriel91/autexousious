use std::fmt;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{ControlAction, ControllerId};

/// Action control for a player.
///
/// This defines the control buttons for the actions.
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct PlayerActionControl {
    /// Control ID of the player.
    pub player: ControllerId,
    /// Game coordinate axis that this controls.
    pub action: ControlAction,
}

impl fmt::Display for PlayerActionControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {} {}", self.player, self.action)
    }
}
