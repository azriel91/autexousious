use collision_loading::CollisionAnimationFrame;
use collision_model::config::{CollisionFrame, Interaction};
use shape_model::Volume;
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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, new)]
pub struct ObjectFrame {
    /// Hittable volume of the object.
    #[serde(flatten)]
    pub sprite: SpriteFrame,
    /// Hittable volume of the object.
    #[serde(flatten)]
    pub collision: CollisionFrame,
}

impl AnimationFrame for ObjectFrame {
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

impl CollisionAnimationFrame for ObjectFrame {
    fn body(&self) -> Option<&Vec<Volume>> {
        self.collision.body.as_ref()
    }

    fn interactions(&self) -> Option<&Vec<Interaction>> {
        self.collision.interactions.as_ref()
    }

    fn wait(&self) -> u32 {
        self.collision.wait
    }
}
