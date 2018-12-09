use amethyst::{core::transform::Transform, ecs::prelude::*};
use derive_new::new;
use object_model::entity::Position;
use typename_derive::TypeName;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct ObjectTransformUpdateSystem;

type ObjectTransformUpdateSystemData<'s> =
    (ReadStorage<'s, Position<f32>>, WriteStorage<'s, Transform>);

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(&mut self, (positions, mut transform_storage): Self::SystemData) {
        for (position, transform) in (&positions, &mut transform_storage).join() {
            // We subtract z from the y translation as the z axis increases "out of the screen".
            // Entities that have a larger Z value are transformed downwards.
            transform.set_x(position.x);
            transform.set_y(position.y - position.z);
            transform.set_z(position.z);
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{nalgebra::Vector3, transform::Transform},
        ecs::prelude::*,
    };
    use amethyst_test::*;
    use object_model::entity::Position;
    use typename::TypeName;

    use super::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz() {
        let setup = |world: &mut World| {
            // Create entity with position
            let position = Position::<f32>::new(-5., -3., -4.);

            let mut transform = Transform::default();
            transform.set_position(Vector3::new(10., 20., 0.));

            let entity = world.create_entity().with(position).with(transform).build();

            world.add_resource(EffectReturn(entity));
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            let store = world.read_storage::<Transform>();

            let mut transform = Transform::default();
            transform.set_position(Vector3::new(-5., 1., -4.));

            assert_eq!(Some(&transform), store.get(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<String, String>()
                .with_system(
                    ObjectTransformUpdateSystem::new(),
                    ObjectTransformUpdateSystem::type_name(),
                    &[]
                ) // kcov-ignore
                .with_setup(setup)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }
}
