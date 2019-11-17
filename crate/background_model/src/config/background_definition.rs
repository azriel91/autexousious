use asset_derive::Asset;
use derive_new::new;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteItem;

/// A grouping of images to draw as a background.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct BackgroundDefinition {
    /// Sprite layers to draw.
    #[serde(default)]
    pub layers: IndexMap<String, SpriteItem>,
}
