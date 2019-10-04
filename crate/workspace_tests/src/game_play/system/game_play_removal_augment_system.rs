#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use amethyst::{
        ecs::{Builder, Entity, Read, SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use amethyst_test::AmethystApplication;
    use asset_model::{config::AssetSlug, loaded::AssetIdMappings};
    use game_play_model::GamePlayEntity;
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::loaded::SequenceId;
    use spawn_model::{loaded::Spawn, play::SpawnEvent};
    use state_registry::StateId;
    use typename::TypeName;

    use game_play::GamePlayRemovalAugmentSystem;

    #[test]
    fn augments_removal_when_state_id_is_game_play() -> Result<(), Error> {
        run_test(StateId::GamePlay, true)
    }

    #[test]
    fn does_not_augment_removal_when_state_id_is_not_game_play() -> Result<(), Error> {
        run_test(StateId::MapSelection, false)
    }

    fn run_test(state_id: StateId, has_removal_expected: bool) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_setup(setup_system_data)
            .with_system(
                GamePlayRemovalAugmentSystem::new(),
                GamePlayRemovalAugmentSystem::type_name(),
                &[],
            )
            .with_resource(state_id)
            .with_effect(spawn_entity)
            .with_assertion(move |world| assert_has_removal(world, has_removal_expected))
            .run()
    }

    fn spawn_entity(world: &mut World) {
        let entity_parent = world.create_entity().build();
        let entity_spawned = world.create_entity().build();
        world.insert(entity_spawned);

        let asset_slug = AssetSlug::from_str("default/fireball")
            .expect("Expected `default/fireball` to be a valid asset slug.");
        let asset_id = {
            let mut asset_id_mappings = world.write_resource::<AssetIdMappings>();
            asset_id_mappings.insert(asset_slug)
        };

        let spawn = Spawn::new(
            asset_id,
            Position::<f32>::new(10., 20., 30.),
            Velocity::<f32>::new(40., 50., 60.),
            SequenceId::new(0),
        );

        send_event(
            world,
            SpawnEvent::new(spawn, entity_parent, entity_spawned, asset_id),
        );
    }

    fn send_event(world: &mut World, spawn_event: SpawnEvent) {
        let mut ec = world.write_resource::<EventChannel<SpawnEvent>>();
        ec.single_write(spawn_event);
    } // kcov-ignore

    fn assert_has_removal(world: &mut World, has_removal: bool) {
        let entity_spawned = *world.read_resource::<Entity>();
        let game_play_entities = world.read_storage::<GamePlayEntity>();
        let game_play_entity_actual = game_play_entities.get(entity_spawned);

        assert_eq!(has_removal, game_play_entity_actual.is_some());
    }

    fn setup_system_data(world: &mut World) {
        <Read<'_, AssetIdMappings> as SystemData>::setup(world);
    }
}
