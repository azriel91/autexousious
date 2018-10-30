use amethyst::renderer::SpriteRender;
use collision_model::animation::CollisionFrameActiveHandle;

/// Animations of the animated components of an object.
#[derive(Clone, Debug, PartialEq)]
pub enum AnimatedComponentDefault {
    /// Sprites.
    SpriteRender(SpriteRender),
    /// Collision.
    CollisionFrame(CollisionFrameActiveHandle),
}
