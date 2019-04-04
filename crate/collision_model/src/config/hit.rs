use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{HitLimit, HitRepeatDelay};

/// Configuration of a hit interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Hit {
    /// Number of ticks to wait before another hit may occur.
    pub repeat_delay: HitRepeatDelay,
    /// Number of objects a `Hit` may collide with.
    pub hit_limit: HitLimit,
    /// Amount of health points (HP) to subtract on collision.
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    pub sp_damage: u32,
}
