use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

use crate::config::CswPortraits;

/// Template for initializing each character selection widget.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CswTemplate {
    /// Portraits to use while character selection is not present.
    pub portraits: CswPortraits,
    /// Layers to render for the character selection widget.
    pub layers: IndexMap<String, UiSpriteLabel>,
}
