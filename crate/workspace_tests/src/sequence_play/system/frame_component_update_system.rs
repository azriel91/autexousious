#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use sequence_model::{
        config::Wait,
        loaded::{SequenceId, WaitSequence, WaitSequenceHandle},
        play::{FrameIndexClock, FrameWaitClock, SequenceUpdateEvent},
    };

    use sequence_play::FrameComponentUpdateSystem;

    #[test]
    fn updates_frame_component_on_sequence_begin_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                // First frame in the sequence.
                frame_index_clock: FrameIndexClock::new_with_value(5, 0),
                frame_wait_clock: FrameWaitClock::new_with_value(5, 0),
                attach_frame_component_data_handle: true,
                sequence_update_events_fn: sequence_begin_events,
            },
            Wait::new(1),
        )
    }

    #[test]
    fn updates_frame_component_on_frame_begin_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                // Third frame in the sequence.
                frame_index_clock: FrameIndexClock::new_with_value(5, 2),
                frame_wait_clock: FrameWaitClock::new_with_value(5, 0),
                attach_frame_component_data_handle: true,
                sequence_update_events_fn: frame_begin_events,
            },
            Wait::new(2),
        )
    }

    #[test]
    fn does_not_panic_when_entity_does_not_have_frame_component_data_handle() -> Result<(), Error> {
        run_test(
            SetupParams {
                // Third frame in the sequence.
                frame_index_clock: FrameIndexClock::new_with_value(5, 2),
                frame_wait_clock: FrameWaitClock::new_with_value(5, 0),
                attach_frame_component_data_handle: false,
                sequence_update_events_fn: frame_begin_events,
            },
            Wait::new(2),
        )
    }

    fn run_test(
        SetupParams {
            frame_index_clock,
            frame_wait_clock,
            attach_frame_component_data_handle,
            sequence_update_events_fn,
        }: SetupParams,
        wait_expected: Wait,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(FrameComponentUpdateSystem::<WaitSequence>::new(), "", &[])
            .with_effect(move |world| {
                let frame_component_data_handle = if attach_frame_component_data_handle {
                    Some(SequenceQueries::wait_sequence_handle(
                        world,
                        &CHAR_BAT_SLUG.clone(),
                        SequenceId::new(1),
                    ))
                } else {
                    None
                };
                initial_values(
                    world,
                    frame_index_clock,
                    frame_wait_clock,
                    frame_component_data_handle,
                )
            })
            .with_effect(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(move |world| {
                // See bat/object.yaml for values.
                expect_component_values(world, wait_expected)
            })
            .run_isolated()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock: FrameIndexClock,
        frame_wait_clock: FrameWaitClock,
        frame_component_data_handle_initial: Option<WaitSequenceHandle>,
    ) {
        let entity = {
            let mut entity_builder = world
                .create_entity()
                .with(frame_index_clock)
                .with(frame_wait_clock)
                .with(Wait::new(2));

            if let Some(frame_component_data_handle_initial) = frame_component_data_handle_initial {
                entity_builder = entity_builder.with(frame_component_data_handle_initial);
            }

            entity_builder.build()
        };

        world.insert(entity);
    }

    fn expect_component_values(world: &mut World, expected_wait: Wait) {
        let entity = *world.read_resource::<Entity>();
        let waits = world.read_storage::<Wait>();

        let wait = waits
            .get(entity)
            .expect("Expected entity to have `Wait` component.");
        assert_eq!(&expected_wait, wait);
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin {
            entity,
            sequence_id: SequenceId(0),
        }]
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        let frame_index = {
            let frame_index_clocks = world.read_storage::<FrameIndexClock>();
            let frame_index_clock = frame_index_clocks
                .get(entity)
                .expect("Expected entity to have `FrameIndexClock` component.");
            (*frame_index_clock).value
        };

        vec![SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index,
        }]
    }

    struct SetupParams {
        frame_index_clock: FrameIndexClock,
        frame_wait_clock: FrameWaitClock,
        attach_frame_component_data_handle: bool,
        sequence_update_events_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
    }
}
