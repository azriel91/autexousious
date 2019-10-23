use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{Layer, MapHeader};

/// Defines a playable area that objects can reside in.
#[derive(Asset, Clone, Debug, Deserialize, Serialize, PartialEq, new)]
pub struct MapDefinition {
    /// Base information of the map.
    pub header: MapHeader,
    /// Image layers to draw.
    #[serde(default)]
    pub layers: Vec<Layer>,
}
