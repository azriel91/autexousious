use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ImpactRepeatDelay;

/// Configuration of an impact interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash, Serialize, new)]
#[serde(default)]
pub struct Impact {
    /// Number of ticks to wait before another impact may occur.
    pub repeat_delay: ImpactRepeatDelay,
    /// Amount of health points (HP) to subtract on collision.
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    pub sp_damage: u32,
}
