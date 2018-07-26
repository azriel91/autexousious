use sprite_loading::AnimationFrame;

/// Animation and interaction information to use on this frame.
///
/// Frames are the level of detail that should carry the following information:
///
/// * **Animation Key Frame:** Sprite sheet, sprite, and transition delay.
/// * **Interaction:** Collision zones, type of interactions.
/// * **Effects:** Sound(s) to play.
/// * **Spawning:** Spawning additional object(s).
/// * **Weapon:** Where an active weapon should be.
#[derive(Clone, Debug, Deserialize, PartialEq, new)]
pub struct Frame {
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
    pub wait: u32,
}

impl AnimationFrame for Frame {
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
