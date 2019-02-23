use collision_model::loaded::{BodySequence, InteractionsSequence};
use sprite_model::loaded::SpriteRenderSequence;

use crate::loaded::WaitSequence;

/// Variants of component sequences of an object.
#[derive(Clone, Debug, PartialEq)]
pub enum ComponentSequence {
    /// Number of ticks to stay on the current frame before switching to the next frame.
    Wait(WaitSequence),
    /// Information for rendering a sprite.
    SpriteRender(SpriteRenderSequence),
    /// Hittable volumes of an interactable object.
    Body(BodySequence),
    /// Effects on other objects.
    Interactions(InteractionsSequence),
}
