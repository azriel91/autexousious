use serde::{Deserialize, Serialize};

/// Direction to switch character selection.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum SwitchDirection {
    /// Switch to previous character.
    Previous,
    /// Switch to next character.
    Next,
}
