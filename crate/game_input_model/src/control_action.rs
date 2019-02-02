#![allow(missing_debug_implementations)] // needed for `EnumIter`

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Control actions for characters.
#[derive(
    Clone, Copy, Debug, Derivative, Deserialize, Display, EnumIter, Hash, PartialEq, Eq, Serialize,
)]
#[derivative(Default)]
pub enum ControlAction {
    /// Defend button.
    Defend,
    /// Jump button.
    Jump,
    /// Attack button.
    #[derivative(Default)]
    Attack,
    /// "Once off" special attacks or infrequent commands.
    Special,
}
