use sprite::SpriteSheetDefinition;

/// Configuration type for all sprite sheet definitions for an object.
#[derive(Debug, Deserialize)]
pub struct SpritesDefinition {
    /// Sprite sheet definitions in the sprites file.
    pub sheets: Vec<SpriteSheetDefinition>,
}
