use config::sprite::SpriteOffset;

/// Information about how sprites are laid out on the sprite sheet.
///
/// This is used to calculate the texture coordinates of each sprite.
#[derive(Constructor, Debug, Deserialize)]
pub struct SpriteSheetDefinition {
    /// Path to the sprite sheet, relative to the object's directory.
    ///
    /// This is a `String` because Amethyst expects an `Into<String>` when loading an asset, and if
    /// we store a `PathBuf`, it would need to re-allocate another `String`.
    pub path: String,
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
    fn default_has_border() -> bool {
        true
    }
}
