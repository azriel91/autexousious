#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Entities, Entity, Read, ReadExpect, World, WorldExt, WriteStorage},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use sequence_model::{
        config::Wait,
        loaded::{WaitSequence, WaitSequenceHandle},
        play::{
            FrameFreezeClock, FrameIndexClock, FrameWaitClock, SequenceStatus, SequenceUpdateEvent,
        },
    };

    use sequence_play::{SequenceUpdateSystem, SequenceUpdateSystemData};

    /// Asserts the following:
    ///
    /// * Resets `FrameIndexClock`.
    /// * Updates the `FrameIndexClock` limit to the new sequence's limit.
    /// * Resets `FrameWaitClock`.
    /// * Updates `FrameWaitClock` limit to the new frame's wait limit.
    /// * No `SequenceUpdateEvent`s are sent.
    #[test]
    fn resets_frame_wait_clocks_on_sequence_begin() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(10, 10),
                    FrameWaitClock::new_with_value(10, 10),
                    None,
                    SequenceStatus::Begin,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 0),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameWaitClock`.
    /// * No change to `FrameWaitClock` limit.
    /// * No `SequenceUpdateEvent`s are sent.
    #[test]
    fn ticks_frame_wait_clock_when_sequence_ongoing_and_no_frame_freeze_clock() -> Result<(), Error>
    {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 0),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 1),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    /// Asserts the following when a frame is still in progress but entity is frozen:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameFreezeClock`.
    /// * No change to `FrameWaitClock` value.
    /// * No change to `FrameWaitClock` limit.
    /// * No `SequenceUpdateEvent`s are sent.
    #[test]
    fn ticks_frame_freeze_clock_when_sequence_ongoing_and_frame_freeze_clock_not_complete(
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 1),
                    Some(FrameFreezeClock::new_with_value(2, 1)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 1),
                    Some(FrameFreezeClock::new_with_value(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    /// Asserts the following when a frame is still in progress but entity is frozen:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * No change to `FrameFreezeClock`.
    /// * Ticks `FrameWaitClock`.
    /// * No change to `FrameWaitClock` limit.
    /// * No `SequenceUpdateEvent`s are sent.
    #[test]
    fn ticks_frame_freeze_clock_when_sequence_ongoing_and_frame_freeze_clock_complete(
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 0),
                    Some(FrameFreezeClock::new_with_value(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 1),
                    Some(FrameFreezeClock::new_with_value(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Resets `FrameWaitClock`.
    /// * Updates `FrameWaitClock` limit to the new frame's wait limit.
    /// * `SequenceUpdateEvent::FrameBegin` events are sent.
    #[test]
    fn resets_frame_wait_clock_and_sends_event_when_frame_ends_and_sequence_ongoing(
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 0),
                    FrameWaitClock::new_with_value(2, 1),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 1),
                    FrameWaitClock::new_with_value(3, 0),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| {
                let events = frame_begin_events(world, 1);
                expect_events(world, events);
            })
            .run_winit_loop()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameWaitClock` value.
    /// * No change to `FrameWaitClock` limit.
    /// * `SequenceUpdateEvent::SequenceEnd` event is sent.
    /// * Sets `SequenceStatus` to `SequenceStatus::End`.
    #[test]
    fn sends_end_event_when_frame_ends_and_sequence_ends() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 4),
                    FrameWaitClock::new_with_value(6, 5),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 5),
                    FrameWaitClock::new_with_value(6, 6),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| {
                let events = sequence_end_events(world, 4);
                expect_events(world, events);
            })
            .run_winit_loop()
    }

    #[test]
    fn does_not_tick_frame_clocks_when_sequence_end() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 5),
                    FrameWaitClock::new_with_value(2, 2),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 5),
                    FrameWaitClock::new_with_value(2, 2),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    #[test]
    fn ticks_frame_freeze_clock_when_sequence_end() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_effect(setup_system_data)
            .with_effect(|world| {
                initial_values(
                    world,
                    FrameIndexClock::new_with_value(5, 5),
                    FrameWaitClock::new_with_value(2, 2),
                    Some(FrameFreezeClock::new_with_value(2, 0)),
                    SequenceStatus::End,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    FrameIndexClock::new_with_value(5, 5),
                    FrameWaitClock::new_with_value(2, 2),
                    Some(FrameFreezeClock::new_with_value(2, 1)),
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_winit_loop()
    }

    fn setup_system_data(world: &mut World) {
        SequenceUpdateSystemData::setup(world);
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock: FrameIndexClock,
        frame_wait_clock: FrameWaitClock,
        frame_freeze_clock: Option<FrameFreezeClock>,
        sequence_status: SequenceStatus,
    ) {
        let wait_sequence_handle = {
            let (loader, wait_sequence_assets) = world
                .system_data::<(ReadExpect<'_, Loader>, Read<'_, AssetStorage<WaitSequence>>)>();

            let wait_sequence = WaitSequence::new(vec![
                Wait::new(2),
                Wait::new(3),
                Wait::new(4),
                Wait::new(5),
                Wait::new(6),
            ]);
            loader.load_from_data(wait_sequence, (), &wait_sequence_assets)
        };

        let entity = {
            let (
                entities,
                mut frame_index_clocks,
                mut frame_wait_clocks,
                mut frame_freeze_clocks,
                mut wait_sequence_handles,
                mut sequence_statuses,
            ) = world.system_data::<TestSystemData>();

            let entity = entities.create();

            frame_index_clocks
                .insert(entity, frame_index_clock)
                .expect("Failed to insert frame_index_clock component.");
            frame_wait_clocks
                .insert(entity, frame_wait_clock)
                .expect("Failed to insert frame_wait_clock component.");
            if let Some(frame_freeze_clock) = frame_freeze_clock {
                frame_freeze_clocks
                    .insert(entity, frame_freeze_clock)
                    .expect("Failed to insert frame_freeze_clock component.");
            }
            wait_sequence_handles
                .insert(entity, wait_sequence_handle)
                .expect("Failed to insert wait_sequence_handle component.");
            sequence_statuses
                .insert(entity, sequence_status)
                .expect("Failed to insert sequence_status component.");

            entity
        };

        world.insert(entity);
    }

    fn expect_values(
        world: &mut World,
        expected_frame_index_clock: FrameIndexClock,
        expected_frame_wait_clock: FrameWaitClock,
        expected_frame_freeze_clock: Option<FrameFreezeClock>,
        expected_sequence_status: SequenceStatus,
    ) {
        let (
            _entities,
            frame_index_clocks,
            frame_wait_clocks,
            frame_freeze_clocks,
            _,
            sequence_statuses,
        ) = world.system_data::<TestSystemData>();

        let entity = *world.read_resource::<Entity>();

        let frame_index_clock = frame_index_clocks
            .get(entity)
            .expect("Expected entity to have frame_index_clock component.");
        let frame_wait_clock = frame_wait_clocks
            .get(entity)
            .expect("Expected entity to have frame_wait_clock component.");
        let frame_freeze_clock = frame_freeze_clocks.get(entity);
        let sequence_status = sequence_statuses
            .get(entity)
            .expect("Expected entity to have sequence_status component.");

        assert_eq!(&expected_frame_index_clock, frame_index_clock);
        assert_eq!(&expected_frame_wait_clock, frame_wait_clock);
        assert_eq!(expected_frame_freeze_clock.as_ref(), frame_freeze_clock);
        assert_eq!(expected_sequence_status, *sequence_status);
    }

    fn expect_events(world: &mut World, expect_events: Vec<SequenceUpdateEvent>) {
        let target_entity = *world.read_resource::<Entity>();
        let mut reader_id = world.write_resource::<ReaderId<SequenceUpdateEvent>>();
        let ec = world.read_resource::<EventChannel<SequenceUpdateEvent>>();

        // Map owned values into references.
        let expect_events = expect_events.iter().collect::<Vec<_>>();

        // Filter events for the entity we care about.
        let actual_events = ec
            .read(&mut reader_id)
            .filter(|ev| match ev {
                SequenceUpdateEvent::SequenceBegin { entity, .. }
                | SequenceUpdateEvent::FrameBegin { entity, .. }
                | SequenceUpdateEvent::SequenceEnd { entity, .. } => target_entity == *entity,
            })
            .collect::<Vec<_>>();

        assert_eq!(expect_events, actual_events)
    }

    fn frame_begin_events(world: &mut World, frame_index: usize) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::FrameBegin {
            entity,
            frame_index,
        }]
    }

    fn sequence_end_events(world: &mut World, frame_index: usize) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceEnd {
            entity,
            frame_index,
        }]
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, FrameWaitClock>,
        WriteStorage<'s, FrameFreezeClock>,
        WriteStorage<'s, WaitSequenceHandle>,
        WriteStorage<'s, SequenceStatus>,
    );
}
