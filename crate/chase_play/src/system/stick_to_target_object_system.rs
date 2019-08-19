use amethyst::{
    core::Transform,
    ecs::{Entities, Join, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use chase_model::play::{ChaseModeStick, TargetObject};
use derivative::Derivative;
use derive_new::new;
use kinematic_model::config::Position;
use typename_derive::TypeName;

/// Updates a `ChaseModeStick` entity's `Position` and `Translation` to match its `TargetObject`.
///
/// If we use the `Parent` component, the child object will inherit all transformations, whereas
/// this will only copy over the **XYZ** coordinates.
#[derive(Debug, Default, TypeName, new)]
pub struct StickToTargetObjectSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct StickToTargetObjectSystemData<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: ReadStorage<'s, TargetObject>,
    /// `ChaseModeStick` components.
    #[derivative(Debug = "ignore")]
    pub chase_mode_sticks: ReadStorage<'s, ChaseModeStick>,
    /// `Position<f32>` components.
    #[derivative(Debug = "ignore")]
    pub positions: WriteStorage<'s, Position<f32>>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
}

impl<'s> System<'s> for StickToTargetObjectSystem {
    type SystemData = StickToTargetObjectSystemData<'s>;

    fn run(
        &mut self,
        StickToTargetObjectSystemData {
            entities,
            target_objects,
            chase_mode_sticks,
            mut positions,
            mut transforms,
        }: Self::SystemData,
    ) {
        (&entities, &target_objects, &chase_mode_sticks)
            .join()
            .for_each(|(child_entity, target_object, _)| {
                let target_position = positions.get(target_object.entity).copied();

                if let Some(target_position) = target_position {
                    if let Some(position) = positions.get_mut(child_entity) {
                        *position = target_position;
                    }
                }

                let target_translation = transforms
                    .get(target_object.entity)
                    .map(Transform::translation)
                    .copied();

                if let Some(translation) = target_translation {
                    if let Some(transform) = transforms.get_mut(child_entity) {
                        *transform.translation_mut() = translation;
                    }
                }
            });
    } // kcov-ignore
}

#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, World},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use chase_model::play::{ChaseModeStick, TargetObject};
    use kinematic_model::config::Position;

    use super::StickToTargetObjectSystem;

    #[test]
    fn updates_child_translation_to_match_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_setup(|world| create_target_and_child_entity(world, true, true, false))
            .with_effect(|world| set_target_translation(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_translation(world, 1., 2., 3.5))
            .run()
    }

    #[test]
    fn updates_child_position_to_match_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_setup(|world| create_target_and_child_entity(world, true, false, true))
            .with_effect(|world| set_target_position(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_position(world, 1., 2., 3.5))
            .run()
    }

    #[test]
    fn does_not_update_components_for_non_chase_mode_stick() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_setup(|world| create_target_and_child_entity(world, false, true, true))
            .with_effect(|world| set_target_translation(world, 1., 2., 3.5))
            .with_effect(|world| set_target_position(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_translation(world, 0., 0., 0.))
            .with_assertion(|world| assert_child_entity_position(world, 0., 0., 0.))
            .run()
    }

    fn create_target_and_child_entity(
        world: &mut World,
        with_chase_mode_stick: bool,
        with_transform: bool,
        with_position: bool,
    ) {
        let target = world.create_entity().with(Transform::default()).build();
        let child = {
            let mut entity_builder = world.create_entity().with(TargetObject::new(target));

            if with_chase_mode_stick {
                entity_builder = entity_builder.with(ChaseModeStick::new());
            }
            if with_transform {
                entity_builder = entity_builder.with(Transform::default());
            }
            if with_position {
                entity_builder = entity_builder.with(Position::<f32>::default());
            }

            entity_builder.build()
        };
        world.add_resource((target, child));
    }

    fn set_target_translation(world: &mut World, x: f32, y: f32, z: f32) {
        let (target, _child) = *world.read_resource::<(Entity, Entity)>();
        let mut transforms = world.write_storage::<Transform>();
        let target_transform = transforms
            .get_mut(target)
            .expect("Expected target entity to have `Transform` component.");

        target_transform.set_translation_xyz(x, y, z);
    }

    fn set_target_position(world: &mut World, x: f32, y: f32, z: f32) {
        let (target, _child) = *world.read_resource::<(Entity, Entity)>();
        let mut positions = world.write_storage::<Position<f32>>();
        positions
            .insert(target, Position::new(x, y, z))
            .expect("Failed to insert `Position<f32>` component.");
    }

    fn assert_child_entity_translation(world: &mut World, x: f32, y: f32, z: f32) {
        let (_target, child) = *world.read_resource::<(Entity, Entity)>();
        let transforms = world.read_storage::<Transform>();
        let child_transform = transforms
            .get(child)
            .expect("Expected child entity to have `Transform` component.");

        assert_eq!(
            &Vector3::new(x.into(), y.into(), z.into()),
            child_transform.translation()
        );
    }

    fn assert_child_entity_position(world: &mut World, x: f32, y: f32, z: f32) {
        let (_target, child) = *world.read_resource::<(Entity, Entity)>();
        let positions = world.read_storage::<Position<f32>>();
        let child_position = positions
            .get(child)
            .expect("Expected child entity to have `Position` component.");

        assert_eq!(&Position::new(x, y, z), child_position);
    }
}
