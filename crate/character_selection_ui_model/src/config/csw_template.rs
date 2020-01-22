use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

use crate::config::{CswLayer, CswPortraits};

/// Template for initializing each asset preview widget.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CswTemplate {
    /// Portraits to use while character selection is not present.
    pub portraits: CswPortraits,
    /// Layers to render for the asset preview widget.
    pub layers: IndexMap<CswLayer, UiSpriteLabel>,
}
