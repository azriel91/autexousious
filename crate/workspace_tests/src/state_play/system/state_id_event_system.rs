#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{SystemData, World, WorldExt},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use state_registry::{StateId, StateIdUpdateEvent};
    use tracker::Prev;

    use state_play::{StateIdEventSystem, StateIdEventSystemData};

    #[test]
    fn does_not_send_event_when_current_state_is_none() -> Result<(), Error> {
        run_test(
            SetupParams {
                state_id: None,
                state_id_prev: None,
            },
            ExpectedParams { events: vec![] },
        )
    }

    #[test]
    fn does_not_send_event_when_current_and_prev_state_same() -> Result<(), Error> {
        run_test(
            SetupParams {
                state_id: Some(StateId::CharacterSelection),
                state_id_prev: Some(StateId::CharacterSelection),
            },
            ExpectedParams { events: vec![] },
        )
    }

    #[test]
    fn sends_event_when_current_and_prev_state_different() -> Result<(), Error> {
        run_test(
            SetupParams {
                state_id: Some(StateId::CharacterSelection),
                state_id_prev: Some(StateId::Loading),
            },
            ExpectedParams {
                events: vec![StateIdUpdateEvent::new(
                    StateId::CharacterSelection,
                    Some(StateId::Loading),
                )],
            },
        )
    }

    #[test]
    fn sends_event_when_current_state_some_and_prev_state_none() -> Result<(), Error> {
        run_test(
            SetupParams {
                state_id: Some(StateId::CharacterSelection),
                state_id_prev: None,
            },
            ExpectedParams {
                events: vec![StateIdUpdateEvent::new(StateId::CharacterSelection, None)],
            },
        )
    }

    fn run_test(
        SetupParams {
            state_id,
            state_id_prev,
        }: SetupParams,
        ExpectedParams { events }: ExpectedParams,
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(StateIdEventSystem::new(), "", &[])
            .with_setup(StateIdEventSystemData::setup)
            .with_setup(setup_event_reader)
            .with_effect(move |world| {
                if let Some(state_id) = state_id {
                    world.insert(state_id);
                }
                if let Some(state_id_prev) = state_id_prev {
                    world.insert(Prev::new(state_id_prev));
                }
            })
            .with_assertion(move |world| {
                assert_events(world, events);
            })
            .run()
    }

    fn setup_event_reader(world: &mut World) {
        let state_id_update_event_rid = {
            let mut state_id_update_ec = world.write_resource::<EventChannel<StateIdUpdateEvent>>();
            state_id_update_ec.register_reader()
        };
        world.insert(state_id_update_event_rid);
    }

    fn assert_events(world: &mut World, events_expected: Vec<StateIdUpdateEvent>) {
        let state_id_update_ec = world.read_resource::<EventChannel<StateIdUpdateEvent>>();
        let mut state_id_update_event_rid = world.write_resource::<ReaderId<StateIdUpdateEvent>>();

        let events_actual = state_id_update_ec
            .read(&mut state_id_update_event_rid)
            .copied()
            .collect::<Vec<StateIdUpdateEvent>>();

        assert_eq!(events_expected, events_actual);
    }

    struct SetupParams {
        state_id: Option<StateId>,
        state_id_prev: Option<StateId>,
    }

    struct ExpectedParams {
        events: Vec<StateIdUpdateEvent>,
    }
}
