use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

/// Portraits to use while character selection is not present.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CswPortraits {
    /// Used when the widget is inactive.
    pub join: UiSpriteLabel,
    /// Used when character selection is "Random".
    pub random: UiSpriteLabel,
}
