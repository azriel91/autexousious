use amethyst::{core::transform::Transform, ecs::prelude::*};
use object_model::entity::Position;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect position have run.
#[derive(Debug, Default, new)]
pub(crate) struct ObjectTransformUpdateSystem;

type ObjectTransformUpdateSystemData<'s> = (ReadStorage<'s, Position>, WriteStorage<'s, Transform>);

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(&mut self, (position_storage, mut transform_storage): Self::SystemData) {
        for (position, mut transform) in (&position_storage, &mut transform_storage).join() {
            transform.translation[0] = position.x;
            transform.translation[1] = position.y + position.z;
        }
    }
}
