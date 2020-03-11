use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ControllerConfig;

/// Structure for holding the input configuration.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct PlayerInputConfig {
    /// Axis control configuration.
    pub name: String,
    /// Axes and action buttons for this player.
    pub controller_config: ControllerConfig,
}
