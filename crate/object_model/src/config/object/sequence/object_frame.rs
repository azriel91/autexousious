use collision_model::config::{Body, Interactions};
use derive_new::new;
use serde::{Deserialize, Serialize};
use sprite_model::config::SpriteRef;

use crate::config::object::Wait;

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
    pub wait: Wait,
    /// Sprite to render.
    pub sprite: SpriteRef,
    /// Hittable volume of the object.
    pub body: Body,
    /// Interaction volumes of the object.
    pub interactions: Interactions,
}
