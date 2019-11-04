use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use typename_derive::TypeName;

/// Game mode menu indicies.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    PartialEq,
    Eq,
    Serialize,
    TypeName,
)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GameModeIndex {
    /// Menu item for starting a game.
    StartGame,
    /// Menu item for exiting the application.
    Exit,
}
