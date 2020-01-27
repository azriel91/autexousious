use asset_model::config::asset_type::Map;
use asset_ui_model::config::AssetSelector;
use serde::{Deserialize, Serialize};

use crate::config::MpwTemplate;

/// Configuration for initializing the map selection UI.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MapSelectionUi {
    /// Template to initialize the map selection widget.
    pub map_preview: MpwTemplate,
    /// Display sheet for all available maps, including `Random`.
    pub maps_available_selector: AssetSelector<Map>,
}
