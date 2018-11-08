use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::SpriteRender};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};

/// Shorthand for `AnimationControlSet<SeqId, SpriteRender>`.
pub type SpriteRenderAcs<SeqId> = AnimationControlSet<SeqId, SpriteRender>;
/// Shorthand for `AnimationControlSet<SeqId, BodyFrameActiveHandle>`.
pub type BodyAcs<SeqId> = AnimationControlSet<SeqId, BodyFrameActiveHandle>;
/// Shorthand for `AnimationControlSet<SeqId, InteractionFrameActiveHandle>`.
pub type InteractionAcs<SeqId> = AnimationControlSet<SeqId, InteractionFrameActiveHandle>;

/// Common game object `Animation` storages.
///
/// These are the storages for the animation control sets common to all game objects.
pub type ObjectAnimationStorages<'s, SeqId> = (
    WriteStorage<'s, SpriteRenderAcs<SeqId>>,
    WriteStorage<'s, BodyAcs<SeqId>>,
    WriteStorage<'s, InteractionAcs<SeqId>>,
);
