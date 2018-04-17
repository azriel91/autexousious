/// Pixel offsets of the sprite relative to the entity's position in the world.
///
/// A positive x value shifts the sprite to the left by that many pixels.
/// A positive y value shifts the sprite upwards by that many pixels.
#[derive(Debug, Deserialize)]
pub struct SpriteOffset {
    /// Number of pixels to shift the sprite to the left, relative to the entity's position.
    pub x: i32,
    /// Number of pixels to shift the sprite upwards, relative to the entity's position.
    pub y: i32,
}
