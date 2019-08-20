use derive_new::new;
use typename_derive::TypeName;

use amethyst::{animation::AnimationControlSet, ecs::prelude::*, renderer::Material};

/// Updates `Character` sequence based on input
#[derive(Debug, Default, TypeName, new)]
pub struct MapAnimationUpdateSystem;

type MapAnimationUpdateSystemData<'s> = (ReadStorage<'s, AnimationControlSet<u32, Material>>,);

impl<'s> System<'s> for MapAnimationUpdateSystem {
    type SystemData = MapAnimationUpdateSystemData<'s>;

    // TODO: Is this system necessary?
    fn run(&mut self, _animation_control_set_storage: Self::SystemData) {} // kcov-ignore
}
