/// Parameters to generate a sprite.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpriteGenParams {
    /// Texture width.
    pub image_w: u32,
    /// Texture height.
    pub image_h: u32,
    /// Individual sprite width.
    pub sprite_w: u32,
    /// Individual sprite height.
    pub sprite_h: u32,
    /// Pixel left coordinate.
    pub pixel_left: u32,
    /// Pixel top coordinate.
    pub pixel_top: u32,
    /// Number of pixels to shift the sprite to the left and down relative to
    /// the entity.
    pub offsets: [f32; 2],
}
