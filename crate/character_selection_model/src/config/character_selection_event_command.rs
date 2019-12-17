use serde::{Deserialize, Serialize};

use crate::config::SwitchDirection;

/// Parameters to instantiate a `CharacterSelectionEvent`.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum CharacterSelectionEventCommand {
    /// Signal to return from `CharacterSelectionState`.
    Return,
    /// Player has joined / become active.
    Join,
    /// Player has left / become inactive.
    Leave,
    /// Character has been switched.
    Switch(SwitchDirection),
    /// Character has been selected.
    Select,
    /// Character has been deselected.
    Deselect,
    /// Character selections have been confirmed.
    Confirm,
}
