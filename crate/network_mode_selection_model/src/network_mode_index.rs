use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Network mode menu indicies.
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, PartialEq, Eq, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NetworkModeIndex {
    /// Host a game.
    Host,
    /// Join a game.
    Join,
    /// Return to the previous menu.
    Back,
}
