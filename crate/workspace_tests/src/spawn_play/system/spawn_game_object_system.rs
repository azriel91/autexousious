#[cfg(test)]
mod tests {
    use std::any;

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::loaded::AssetId;
    use assets_test::{CHAR_BAT_SLUG, ENERGY_SQUARE_SLUG};
    use kinematic_model::config::{Position, Velocity};
    use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};
    use spawn_model::{
        loaded::{Spawn, Spawns},
        play::SpawnEvent,
    };

    use spawn_play::SpawnGameObjectSystem;

    #[test]
    fn spawns_entity_for_sequence_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::SequenceBegin {
                entity,
                sequence_id: SequenceId::new(0),
            }),
            2,
        )
    }

    #[test]
    fn spawns_entity_for_frame_begin_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::FrameBegin {
                entity,
                frame_index: 0,
            }),
            2,
        )
    }

    #[test]
    fn does_not_spawn_entity_for_sequence_end_events() -> Result<(), Error> {
        run_test(
            Some(|entity| SequenceUpdateEvent::SequenceEnd {
                entity,
                frame_index: 0,
            }),
            0,
        )
    }

    #[test]
    fn does_not_spawn_entity_when_no_sequence_update_event() -> Result<(), Error> {
        run_test(None, 0)
    }

    fn run_test(
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
        spawn_count_expected: usize,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                SpawnGameObjectSystem::new(),
                any::type_name::<SpawnGameObjectSystem>(),
                &[],
            )
            .with_effect(setup_spawn_ec_reader)
            .with_assertion(|world| {
                assert_object_count(world, 0);
                assert_events(world, 0);
            })
            .with_effect(move |world| create_entity_with_spawns(world, sequence_update_event_fn))
            .with_assertion(move |world| {
                assert_object_count(world, spawn_count_expected);
                assert_events(world, spawn_count_expected);
            })
            .run_isolated()
    }

    fn setup_spawn_ec_reader(world: &mut World) {
        let spawn_event_rid = world
            .write_resource::<EventChannel<SpawnEvent>>()
            .register_reader(); // kcov-ignore

        world.insert(spawn_event_rid);
    }

    fn create_entity_with_spawns(
        world: &mut World,
        sequence_update_event_fn: Option<fn(Entity) -> SequenceUpdateEvent>,
    ) {
        let spawns_handle = {
            let (loader, spawns_assets) =
                world.system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<Spawns>>)>();
            loader.load_from_data(
                Spawns::new(vec![spawn_character(world), spawn_energy(world)]),
                (),
                &spawns_assets,
            )
        };

        let entity = world.create_entity().with(spawns_handle).build();
        world.insert(entity);

        if let Some(sequence_update_event_fn) = sequence_update_event_fn {
            let mut sequence_update_ec =
                world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            let sequence_update_event = sequence_update_event_fn(entity);
            sequence_update_ec.single_write(sequence_update_event);
        }
    }

    fn spawn_character(world: &World) -> Spawn {
        let asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
        Spawn::new(
            asset_id,
            Position::<f32>::new(0., 0., 0.),
            Velocity::<f32>::new(0., 0., 0.),
            SequenceId::new(0),
        )
    }

    fn spawn_energy(world: &World) -> Spawn {
        let asset_id = AssetQueries::id(world, &*ENERGY_SQUARE_SLUG);
        Spawn::new(
            asset_id,
            Position::<f32>::new(0., 0., 0.),
            Velocity::<f32>::new(0., 0., 0.),
            SequenceId::new(0),
        )
    }

    fn assert_object_count(world: &mut World, count: usize) {
        let asset_ids = world.read_storage::<AssetId>();
        assert_eq!(count, asset_ids.count());
    }

    fn assert_events(world: &mut World, event_count: usize) {
        let mut spawn_event_rid = &mut world.write_resource::<ReaderId<SpawnEvent>>();

        let spawn_ec = world.read_resource::<EventChannel<SpawnEvent>>();
        let actual_events = spawn_ec
            .read(&mut spawn_event_rid)
            .collect::<Vec<&SpawnEvent>>();

        assert_eq!(event_count, actual_events.len());

        if event_count > 0 {
            let spawns_expected = vec![spawn_character(world), spawn_energy(world)];
            let entity_parent = *world.read_resource::<Entity>();
            spawns_expected
                .into_iter()
                .zip(actual_events.into_iter())
                .for_each(|(spawn_expected, ev)| {
                    assert_eq!(&spawn_expected, &ev.spawn);
                    assert_eq!(entity_parent, ev.entity_parent);
                });
        }
    }
}
