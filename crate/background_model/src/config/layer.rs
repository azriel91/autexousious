use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{LayerFrame, LayerPosition};

/// An image layer on a background.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, new)]
pub struct Layer {
    /// Position of the image on the background.
    #[serde(default)]
    pub position: LayerPosition,
    /// Frames in the animation sequence.
    pub frames: Vec<LayerFrame>,
}
