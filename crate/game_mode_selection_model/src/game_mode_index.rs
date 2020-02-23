use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Game mode menu indicies.
#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumIter, EnumString, PartialEq, Eq, Serialize,
)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GameModeIndex {
    /// Starts a local game.
    StartGame,
    /// Goes to the network mode selection menu.
    NetworkPlay,
    /// Opens control settings.
    ControlSettings,
    /// Exits the application.
    Exit,
}
