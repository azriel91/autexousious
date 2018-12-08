use derive_new::new;

/// Animation frame that displays a sprite.
#[derive(Clone, Copy, Debug, Deserialize, Hash, PartialEq, Eq, new)]
pub struct SpriteFrame {
    /// Sprite sheet number.
    ///
    /// Note: this will almost always differ from the sheet number when loaded into Amethyst.
    ///
    /// Amethyst uses a global texture id map, so this number will be relative to the offset
    /// allocated to the object that this sprite sheet belongs to.
    pub sheet: usize,
    /// Sprite number on the sprite sheet.
    pub sprite: usize,
    /// Number of ticks to wait before the sequence switches to the next frame.
    #[serde(default)]
    pub wait: u32,
}
