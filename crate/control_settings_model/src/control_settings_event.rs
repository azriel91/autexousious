use serde::{Deserialize, Serialize};

/// Event signalling a change in the `ControlSettings` state.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ControlSettingsEvent {
    /// Returns to the previous menu.
    Return,
    /// Control settings should be reloaded.
    ReloadRequest,
}
