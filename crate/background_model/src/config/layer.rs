use derive_new::new;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteFrame;

use crate::config::LayerPosition;

/// An image layer on a background.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct Layer {
    /// Position of the image on the background.
    #[serde(default)]
    pub position: LayerPosition,
    /// Frames in the animation sequence.
    pub frames: Vec<SpriteFrame>,
}
