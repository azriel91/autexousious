use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{UiSequences, UiType};

/// Definition for a `State`'s UI.
#[derive(Asset, Clone, Debug, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields)]
pub struct UiDefinition {
    /// Type of UI -- generic menu, character selection, map selection.
    #[serde(flatten)]
    pub ui_type: UiType,
    /// Sequences used by the UI type.
    #[serde(default)]
    pub sequences: UiSequences,
}
