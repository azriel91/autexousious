use derive_new::new;
use game_input_model::loaded::PlayerControllers;
use serde::{Deserialize, Serialize};

use crate::play::SessionDevice;

/// Message when a device joins the current session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SessionDeviceJoin {
    /// The device that joined the session.
    pub session_device: SessionDevice,
    /// All player controllers.
    pub player_controllers: PlayerControllers,
}
