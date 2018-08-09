use amethyst::{
    animation::AnimationControlSet,
    core::{GlobalTransform, Transform},
    ecs::prelude::*,
    renderer::{SpriteRender, Transparent},
};

/// Map layer `Component` storages.
pub type MapLayerComponentStorages<'s> = (
    WriteStorage<'s, SpriteRender>,
    WriteStorage<'s, Transparent>,
    WriteStorage<'s, Transform>,
    WriteStorage<'s, GlobalTransform>,
    WriteStorage<'s, AnimationControlSet<u32, SpriteRender>>,
);
