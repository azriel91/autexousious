use derive_new::new;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

/// Defines text to display.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct UiLabel {
    /// Position of the label relative to its parent.
    pub position: PositionInit,
    /// Text to display.
    pub text: String,
}
