use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::{LayerFrame, Position};

/// An image layer on a map.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, new)]
pub struct Layer {
    /// Position of the image on the map.
    #[serde(default)]
    pub position: Position,
    /// Key frames in the animation sequence.
    pub frames: Vec<LayerFrame>,
}
