use amethyst::{
    core::Transform,
    ecs::prelude::*,
    renderer::{SpriteRender, Transparent},
};
use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
use object_model::entity::Kinematics;

/// Common game object `Component` storages.
///
/// These are the storages for the components common to all game objects.
pub type ObjectComponentStorages<'s> = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Transparent>,
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, BodyFrameActiveHandle>,
    WriteStorage<'s, InteractionFrameActiveHandle>,
);
