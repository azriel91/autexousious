use amethyst::{animation::Animation, assets::Handle};

use animation::CollisionFrameActiveHandle;

/// Type alias for collision volume animation handles.
pub type CollisionAnimationHandle = Handle<Animation<CollisionFrameActiveHandle>>;
