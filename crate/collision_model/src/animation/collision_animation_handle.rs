use amethyst::{animation::Animation, assets::Handle};

use config::CollisionFrame;

/// Type alias for collision volume animation handles.
pub type CollisionAnimationHandle = Handle<Animation<&'static CollisionFrame>>;
