use serde::{Deserialize, Serialize};

use crate::config::AssetSwitch;

/// Parameters to instantiate a `AssetSelectionEvent`.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AssetSelectionEventCommand {
    /// Signal to return from the current `State`.
    Return,
    /// Player has joined / become active.
    Join,
    /// Player has left / become inactive.
    Leave,
    /// Asset has been switched.
    Switch(AssetSwitch),
    /// Asset has been selected.
    Select,
    /// Asset has been deselected.
    Deselect,
    /// Asset selections have been confirmed.
    Confirm,
}
