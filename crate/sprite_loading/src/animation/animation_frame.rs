use sprite_model::config::SpriteFrame;

/// Information for a single animation frame.
///
/// Animation frames carry the following information:
///
/// * **Texture index:** Index of the material that holds the loaded sprite sheet texture.
/// * **Sprite index:** Index of the sprite's texture offset coordinates.
/// * **Wait:** Number of ticks to wait before the animation switches to the next frame.
pub trait AnimationFrame {
    /// Returns the texture index in the `MaterialTextureSet`.
    fn texture_index(&self) -> usize;
    /// Returns the index of the sprite's texture offset coordinates.
    fn sprite_index(&self) -> usize;
    /// Returns the number of ticks to wait before the animation switches to the next frame.
    fn wait(&self) -> u32;
}

impl AnimationFrame for SpriteFrame {
    fn texture_index(&self) -> usize {
        self.sheet
    }

    fn sprite_index(&self) -> usize {
        self.sprite
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}
