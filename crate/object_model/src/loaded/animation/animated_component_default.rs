use amethyst::renderer::SpriteRender;
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};

/// Animations of the animated components of an object.
#[derive(Clone, Debug, PartialEq)]
pub enum AnimatedComponentDefault {
    /// Sprites.
    SpriteRender(SpriteRender),
    /// Body.
    BodyFrame(BodyFrameActiveHandle),
    /// Interaction.
    InteractionFrame(InteractionFrameActiveHandle),
}
