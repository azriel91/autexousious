use amethyst::{
    animation::AnimationControlSet,
    core::{GlobalTransform, Transform},
    ecs::prelude::*,
    renderer::SpriteRender,
};
use object_model::entity::Kinematics;

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
pub type ObjectComponentStorages<'s, SeqId> = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, GlobalTransform>,
    WriteStorage<'s, AnimationControlSet<SeqId, SpriteRender>>,
);
