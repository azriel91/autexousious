use collision_loading::{BodyAnimationFrame, InteractionAnimationFrame};
use collision_model::config::{BodyFrame, Interaction, InteractionFrame};
use derive_new::new;
use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
pub struct ObjectFrame {
    /// Sprite to render.
    #[serde(flatten)]
    pub sprite: SpriteFrame,
    /// Hittable volume of the object.
    #[serde(flatten)]
    pub body: BodyFrame,
    /// Interaction volumes of the object.
    #[serde(flatten)]
    pub interaction: InteractionFrame,
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

impl BodyAnimationFrame for ObjectFrame {
    fn body(&self) -> Option<&Vec<Volume>> {
        self.body.body.as_ref()
    }

    fn wait(&self) -> u32 {
        self.body.wait
    }
}

impl InteractionAnimationFrame for ObjectFrame {
    fn interactions(&self) -> Option<&Vec<Interaction>> {
        self.interaction.interactions.as_ref()
    }

    fn wait(&self) -> u32 {
        self.body.wait
    }
}
