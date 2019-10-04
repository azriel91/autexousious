#[cfg(test)]
mod tests {
    use amethyst::{
        core::{math::Vector3, transform::Transform},
        ecs::{Builder, Entity, World, WorldExt},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use chase_model::play::{ChaseModeStick, TargetObject};
    use kinematic_model::config::Position;

    use chase_play::StickToTargetObjectSystem;

    #[test]
    fn updates_child_translation_to_match_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_effect(|world| create_target_and_child_entity(world, true, true, false))
            .with_effect(|world| set_target_translation(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_translation(world, 1., 2., 3.5))
            .run()
    }

    #[test]
    fn updates_child_position_to_match_target() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_effect(|world| create_target_and_child_entity(world, true, false, true))
            .with_effect(|world| set_target_position(world, 1., 2., 3.5))
            .with_assertion(|world| assert_child_entity_position(world, 1., 2., 3.5))
            .run()
    }

    #[test]
    fn does_not_update_components_for_non_chase_mode_stick() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StickToTargetObjectSystem::new(), "", &[])
            .with_effect(|world| create_target_and_child_entity(world, false, true, true))
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
        world.insert((target, child));
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
