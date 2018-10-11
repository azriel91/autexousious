use amethyst::{animation::Animation, assets::Handle, renderer::SpriteRender};

/// Type alias for sprite render animation handles.
pub type SpriteAnimationHandle = Handle<Animation<SpriteRender>>;
