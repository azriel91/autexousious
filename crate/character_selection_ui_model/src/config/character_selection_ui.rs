use asset_ui_model::config::AssetDisplay;
use serde::{Deserialize, Serialize};

use crate::config::{CswDefinition, CswTemplate};

/// Configuration for initializing the character selection UI.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CharacterSelectionUi {
    /// Positions of the character selection widgets on screen.
    pub widgets: Vec<CswDefinition>,
    /// Template to initialize each widget with.
    pub widget_template: CswTemplate,
    /// Display sheet for all available characters, including `Random`.
    pub characters_available_display: AssetDisplay,
}
