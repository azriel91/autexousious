use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::SpriteRender};

/// Common game object `Animation` storages.
///
/// These are the storages for the animation control sets common to all game objects.
pub type ObjectAnimationStorages<'s, SeqId> =
    (WriteStorage<'s, AnimationControlSet<SeqId, SpriteRender>>,);
