use collision_model::animation::CollisionAnimationHandle;
use sprite_loading::SpriteAnimationHandle;

/// Animations of the animated components of an object.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AnimatedComponentAnimation {
    /// Sprites.
    SpriteRender(SpriteAnimationHandle),
    /// Collision
    Collision(CollisionAnimationHandle),
}
