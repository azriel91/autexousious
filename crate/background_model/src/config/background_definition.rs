use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::Layer;

/// A grouping of images to draw as a background.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct BackgroundDefinition {
    /// Image layers to draw.
    #[serde(default)]
    pub layers: Vec<Layer>,
}
