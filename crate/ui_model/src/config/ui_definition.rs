use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};
use ui_button_model::config::UiButtons;

use crate::config::{UiSequences, UiType};

/// Definition for a `State`'s UI.
#[derive(Asset, Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiDefinition {
    /// Type of UI -- generic menu, character selection, map selection.
    #[serde(flatten)]
    pub ui_type: UiType,
    /// Buttons in the UI.
    #[serde(default)]
    pub buttons: UiButtons,
    /// Whether or not to display mini control settings.
    #[serde(default)]
    pub display_control_buttons: bool,
    /// Sequences used by the UI type.
    #[serde(default)]
    pub sequences: UiSequences,
}
