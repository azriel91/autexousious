use amethyst::ecs::prelude::*;
use derive_new::new;
use object_model::entity::{Position, Velocity};
use typename_derive::TypeName;

/// Updates each entity's `Position` based on their `Velocity` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectKinematicsUpdateSystem;

type ObjectKinematicsUpdateSystemData<'s> = (
    WriteStorage<'s, Position<f32>>,
    WriteStorage<'s, Velocity<f32>>,
);

impl<'s> System<'s> for ObjectKinematicsUpdateSystem {
    type SystemData = ObjectKinematicsUpdateSystemData<'s>;

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.0 += velocity.0;
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::prelude::*;
    use amethyst_test::*;
    use object_model::entity::{Position, Velocity};
    use typename::TypeName;

    use super::ObjectKinematicsUpdateSystem;

    #[test]
    fn adds_velocity_to_position() {
        let setup = |world: &mut World| {
            // Create entity with kinematics
            let position = Position::<f32>::new(-2., -2., -2.);
            let velocity = Velocity::<f32>::new(-3., -3., -3.);
            let entity = world.create_entity().with(position).with(velocity).build();

            world.add_resource(EffectReturn(entity));
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            let positions = world.read_storage::<Position<f32>>();
            let velocities = world.read_storage::<Velocity<f32>>();

            let position = Position::new(-5., -5., -5.);
            let velocity = Velocity::new(-3., -3., -3.);

            assert_eq!(Some(&position), positions.get(entity));
            assert_eq!(Some(&velocity), velocities.get(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_system(
                    ObjectKinematicsUpdateSystem::new(),
                    ObjectKinematicsUpdateSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_setup(setup)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }
}
