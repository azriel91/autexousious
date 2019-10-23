use derive_new::new;
use serde::{Deserialize, Serialize};

/// Position of a layer on a background.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize, new)]
#[serde(default)]
pub struct LayerPosition {
    /// X coordinate of the image on the background.
    pub x: i32,
    /// Y coordinate of the image on the background.
    pub y: i32,
    /// Z coordinate of the image on the background.
    pub z: i32,
}
