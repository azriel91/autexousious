use asset_derive::Asset;
use background_model::config::BackgroundDefinition;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::MapHeader;

/// Defines a playable area that objects can reside in.
#[derive(Asset, Clone, Debug, Deserialize, Serialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
    /// Background to draw.
    #[serde(flatten)]
    pub background: BackgroundDefinition,
}
