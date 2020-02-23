use game_mode_selection_model::GameModeIndex;
use network_mode_selection_model::NetworkModeIndex;
use serde::{Deserialize, Serialize};

/// Sum type of all menu index types.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum MenuIndex {
    /// Game mode menu indicies.
    GameMode(GameModeIndex),
    /// Network mode menu indicies.
    NetworkMode(NetworkModeIndex),
}
