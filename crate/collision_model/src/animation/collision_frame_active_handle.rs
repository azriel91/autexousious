use animation_support::ActiveHandle;

use animation::CollisionFrameId;
use config::CollisionFrame;

/// `ActiveHandle` to animate `CollisionFrame`s.
pub type CollisionFrameActiveHandle = ActiveHandle<CollisionFrameId, CollisionFrame>;
