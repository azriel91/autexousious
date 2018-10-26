use animation_support::AnimationDataSet;

use animation::CollisionFrameId;
use config::CollisionFrame;

/// Map of collision frame ID to `&CollisionFrame` to enable animation.
pub type CollisionDataSet<'f> = AnimationDataSet<CollisionFrameId, &'f CollisionFrame>;
