use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ImpactRepeatDelay;

/// Configuration of an impact interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash, Serialize, new)]
pub struct Impact {
    /// Number of ticks to wait before another impact may occur.
    pub repeat_delay: ImpactRepeatDelay,
}
