use derive_new::new;
use serde::{Deserialize, Serialize};

/// Pixel offsets of the sprite relative to the entity's position in the world.
///
/// A positive x value shifts the sprite to the left by that many pixels.
/// A positive y value shifts the sprite upwards by that many pixels.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SpriteOffset {
    /// Number of pixels to shift the sprite to the left, relative to the
    /// entity's position.
    pub x: i32,
    /// Number of pixels to shift the sprite upwards, relative to the entity's
    /// position.
    pub y: i32,
}

impl From<(i32, i32)> for SpriteOffset {
    fn from((x, y): (i32, i32)) -> Self {
        SpriteOffset::new(x, y)
    }
}
