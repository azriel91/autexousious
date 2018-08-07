use amethyst::{
    animation::AnimationControlSet,
    core::{GlobalTransform, Transform},
    ecs::prelude::*,
    renderer::SpriteRender,
};

/// Map layer `Component` storages.
pub type MapLayerComponentStorages<'s> = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, GlobalTransform>,
    WriteStorage<'s, AnimationControlSet<u32, SpriteRender>>,
);
