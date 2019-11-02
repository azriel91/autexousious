use asset_derive::Asset;
use derive_new::new;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteSequence;

/// A grouping of images to draw as a background.
#[derive(Asset, Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
pub struct BackgroundDefinition {
    /// Sprite layers to draw.
    #[serde(default)]
    pub layers: Vec<SpriteSequence>,
}
