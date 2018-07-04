use amethyst::{core::transform::Transform, ecs::prelude::*};
use object_model::entity::Kinematics;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, new)]
pub(crate) struct ObjectTransformUpdateSystem;

type ObjectTransformUpdateSystemData<'s> = (
    ReadStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, Transform>,
);

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(&mut self, (kinematics_storage, mut transform_storage): Self::SystemData) {
        for (kinematics, mut transform) in (&kinematics_storage, &mut transform_storage).join() {
            let position = &kinematics.position;
            transform.translation[0] = position.x;
            transform.translation[1] = position.y + position.z;
        }
    }
}
