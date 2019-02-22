use collision_loading::InteractionAnimationFrame;
use collision_model::{
    animation::BodyAnimationFrame,
    config::{Body, Interaction, Interactions},
};
use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;
use sprite_loading::AnimationFrame;
use sprite_model::config::SpriteRef;

/// Animation and interaction information to use on this frame.
///
/// Frames are the level of detail that should carry the following information:
///
/// * **Animation Key Frame:** Sprite sheet, sprite, and transition delay.
/// * **Interaction:** Collision zones, type of interactions.
/// * **Effects:** Sound(s) to play.
/// * **Spawning:** Spawning additional object(s).
/// * **Weapon:** Where an active weapon should be.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(default)]
pub struct ObjectFrame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: u32,
    /// Sprite to render.
    pub sprite: SpriteRef,
    /// Hittable volume of the object.
    pub body: Body,
    /// Interaction volumes of the object.
    pub interactions: Interactions,
}

impl AnimationFrame for ObjectFrame {
    fn texture_index(&self) -> usize {
        self.sprite.sheet
    }

    fn sprite_index(&self) -> usize {
        self.sprite.index
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}

impl BodyAnimationFrame for ObjectFrame {
    fn body(&self) -> &Vec<Volume> {
        &*self.body
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}

impl InteractionAnimationFrame for ObjectFrame {
    fn interactions(&self) -> &Vec<Interaction> {
        &*self.interactions.as_ref()
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}
