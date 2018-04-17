use std::path::PathBuf;

use sprite::SpriteOffset;

/// Information about how sprites are laid out on the sprite sheet.
///
/// This is used to calculate the texture coordinates of each sprite.
#[derive(Debug, Deserialize)]
pub struct SpriteSheetDefinition {
    /// Path to the sprite sheet, relative to the object's directory.
    pub path: PathBuf,
    /// Width of each individual sprite on the sprite sheet.
    pub sprite_w: f32,
    /// Height of each individual sprite on the sprite sheet.
    pub sprite_h: f32,
    /// Number of rows in the sprite sheet.
    ///
    /// This is the number of sprites counting down the sheet.
    pub row_count: usize,
    /// Number of columns in the sprite sheet.
    ///
    /// This is the number of sprites counting across the sheet.
    pub column_count: usize,
    /// Whether or not there is a 1 pixel border between sprites.
    #[serde(default = "SpriteSheetDefinition::default_has_border")]
    pub has_border: bool,
    /// Pixel offsets of the sprite relative to the entity's position in the world.
    ///
    /// A positive x value shifts the sprite to the left by that many pixels.
    /// A positive y value shifts the sprite upwards by that many pixels.
    pub offsets: Vec<SpriteOffset>,
}

impl SpriteSheetDefinition {
    /// Returns a new sprite sheet definition.
    ///
    /// # Parameters:
    ///
    /// * `path`: Path to the sprite sheet, relative to the object's directory.
    /// * `sprite_w`: Width of each individual sprite on the sprite sheet.
    /// * `sprite_h`: Height of each individual sprite on the sprite sheet.
    /// * `row_count`: Number of rows in the sprite sheet.
    /// * `column_count`: Number of columns in the sprite sheet.
    /// * `has_border`: Whether or not there is a 1 pixel border between sprites.
    pub fn new(
        path: PathBuf,
        sprite_w: f32,
        sprite_h: f32,
        row_count: usize,
        column_count: usize,
        has_border: bool,
        offsets: Vec<SpriteOffset>,
    ) -> Self {
        SpriteSheetDefinition {
            path,
            sprite_w,
            sprite_h,
            row_count,
            column_count,
            has_border,
            offsets,
        }
    }

    fn default_has_border() -> bool {
        true
    }
}
