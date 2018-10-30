use amethyst::assets::Handle;
use animation_support::AnimationDataSet;

use animation::CollisionFrameId;
use config::CollisionFrame;

/// Map of collision frame ID to `Handle<CollisionFrame>` to enable animation.
pub type CollisionDataSet = AnimationDataSet<CollisionFrameId, Handle<CollisionFrame>>;
