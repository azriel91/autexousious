use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::SpriteRender};
use collision_model::animation::CollisionFrameActiveHandle;

/// Shorthand for `AnimationControlSet<SeqId, SpriteRender>`.
pub type SpriteRenderAcs<SeqId> = AnimationControlSet<SeqId, SpriteRender>;
/// Shorthand for `AnimationControlSet<SeqId, CollisionFrameActiveHandle>`.
pub type CollisionAcs<SeqId> = AnimationControlSet<SeqId, CollisionFrameActiveHandle>;

/// Common game object `Animation` storages.
///
/// These are the storages for the animation control sets common to all game objects.
pub type ObjectAnimationStorages<'s, SeqId> = (
    WriteStorage<'s, SpriteRenderAcs<SeqId>>,
    WriteStorage<'s, CollisionAcs<SeqId>>,
);
