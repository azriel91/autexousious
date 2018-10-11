use collision_model::config::CollisionFrame;
use sprite_loading::AnimationFrame;
use sprite_model::config::SpriteFrame;

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
    /// Hittable volume of the object.
    #[serde(flatten)]
    pub sprite: SpriteFrame,
    /// Hittable volume of the object.
    #[serde(flatten)]
    pub collision: CollisionFrame,
}

impl AnimationFrame for Frame {
    fn texture_index(&self) -> usize {
        self.sprite.sheet
    }

    fn sprite_index(&self) -> usize {
        self.sprite.sprite
    }

    fn wait(&self) -> u32 {
        self.sprite.wait
    }
}
