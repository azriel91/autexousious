use derive_new::new;

use crate::{Axis, ControllerId};

/// Axis control for a player.
#[derive(Clone, Copy, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct PlayerAxisControl {
    /// Control ID of the player.
    pub player: ControllerId,
    /// Game coordinate axis that this controls.
    pub axis: Axis,
}
