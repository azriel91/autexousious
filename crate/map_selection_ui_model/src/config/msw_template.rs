use indexmap::IndexMap;
use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};
use ui_label_model::config::UiSpriteLabel;

use crate::config::{MswLayer, MswPortraits};

/// Configuration for initializing the map selection preview.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MswTemplate {
    /// Position of the map selection widget on screen.
    pub position: PositionInit,
    /// Portraits to use while map selection is not present.
    pub portraits: MswPortraits,
    /// Layers to render for the map selection preview.
    pub layers: IndexMap<MswLayer, UiSpriteLabel>,
}
