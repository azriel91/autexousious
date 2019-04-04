use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::HitRepeatDelay;

/// Configuration of a hit interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Hit {
    /// Number of ticks to wait before another hit may occur.
    pub repeat_delay: HitRepeatDelay,
    /// Amount of health points (HP) to subtract on collision.
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    pub sp_damage: u32,
}
