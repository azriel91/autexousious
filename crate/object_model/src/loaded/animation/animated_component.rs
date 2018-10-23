use sprite_loading::SpriteAnimationHandle;

/// Animated components of an object.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AnimatedComponent {
    /// Sprites.
    SpriteRender(SpriteAnimationHandle),
}
