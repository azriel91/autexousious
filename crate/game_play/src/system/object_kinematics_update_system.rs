use amethyst::ecs::prelude::*;
use object_model::entity::Kinematics;

/// Updates each entity's `Position` based on their `Velocity` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, new)]
pub(crate) struct ObjectKinematicsUpdateSystem;

type ObjectKinematicsUpdateSystemData<'s> = WriteStorage<'s, Kinematics<f32>>;

impl<'s> System<'s> for ObjectKinematicsUpdateSystem {
    type SystemData = ObjectKinematicsUpdateSystemData<'s>;

    fn run(&mut self, mut kinematics_storage: Self::SystemData) {
        for mut kinematics in (&mut kinematics_storage).join() {
            *kinematics.position += *kinematics.velocity;
        }
    }
}
