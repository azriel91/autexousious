#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{SystemData, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use sequence_model::config::Wait;
    use state_registry::{StateId, StateIdUpdateEvent};

    use state_play::{StateBackgroundSpawnSystem, StateBackgroundSpawnSystemData};

    #[test]
    fn does_not_spawn_entities_when_no_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![],
                events_next: vec![],
            },
            ExpectedParams {
                entity_count: 0,
                entity_count_next: 0,
            },
        )
    }

    #[test]
    fn spawns_layer_entities_when_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
                events_next: vec![],
            },
            ExpectedParams {
                // See `assets_test/assets/test/character_selection/background.yaml`.
                entity_count: 1,
                entity_count_next: 1,
            },
        )
    }

    #[test]
    fn deletes_previous_layer_entities_when_event_sent() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
                events_next: vec![StateIdUpdateEvent::new(
                    StateId::Loading,
                    Some(StateId::CharacterSelection),
                )],
            },
            ExpectedParams {
                // See `assets_test/assets/test/character_selection/background.yaml`.
                entity_count: 1,
                // See `assets_test/assets/test/loading/background.yaml`.
                entity_count_next: 0,
            },
        )
    }

    #[test]
    fn deletes_previous_layer_entities_when_next_state_id_has_no_assets() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
                events_next: vec![StateIdUpdateEvent::new(
                    StateId::GamePlay,
                    Some(StateId::CharacterSelection),
                )],
            },
            ExpectedParams {
                // See `assets_test/assets/test/character_selection/background.yaml`.
                entity_count: 1,
                entity_count_next: 0,
            },
        )
    }

    #[test]
    fn only_processes_last_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                events: vec![
                    StateIdUpdateEvent::new(StateId::CharacterSelection, None),
                    StateIdUpdateEvent::new(StateId::Loading, Some(StateId::CharacterSelection)),
                ],
                events_next: vec![],
            },
            ExpectedParams {
                entity_count: 0,
                entity_count_next: 0,
            },
        )
    }

    fn run_test(
        SetupParams {
            events,
            events_next,
        }: SetupParams,
        ExpectedParams {
            entity_count,
            entity_count_next,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(StateBackgroundSpawnSystem::new(), "", &[])
            .with_setup(StateBackgroundSpawnSystemData::setup)
            .with_effect(move |world| {
                send_events(world, events);
            })
            .with_assertion(move |world| {
                assert_background_entity_count(world, entity_count);
            })
            .with_effect(move |world| {
                send_events(world, events_next);
            })
            .with_effect(|_| {}) // Wait for one tick to allow entities to be deleted.
            .with_assertion(move |world| {
                assert_background_entity_count(world, entity_count_next);
            })
            .run()
    }

    fn send_events(world: &mut World, mut events: Vec<StateIdUpdateEvent>) {
        let mut state_id_update_ec = world.write_resource::<EventChannel<StateIdUpdateEvent>>();
        state_id_update_ec.iter_write(events.drain(..));
    }

    fn assert_background_entity_count(world: &mut World, entity_count: usize) {
        let waits = world.read_storage::<Wait>();

        assert_eq!(entity_count, waits.count());
    }

    struct SetupParams {
        events: Vec<StateIdUpdateEvent>,
        events_next: Vec<StateIdUpdateEvent>,
    }

    struct ExpectedParams {
        entity_count: usize,
        entity_count_next: usize,
    }
}
