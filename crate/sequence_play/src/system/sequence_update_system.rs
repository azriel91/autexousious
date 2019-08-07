use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use sequence_model::{
    loaded::{WaitSequence, WaitSequenceHandle},
    play::{
        FrameFreezeClock, FrameIndexClock, FrameWaitClock, SequenceStatus, SequenceUpdateEvent,
    },
};
use shred_derive::SystemData;
use typename_derive::TypeName;

/// Ticks the logic clocks for sequences, and sends `SequenceUpdateEvent`s.
///
/// The logic clocks include:
///
/// * `FrameFreezeClock`
/// * `FrameWaitClock`
/// * `FrameIndexClock`
///
/// This system **must** be run before all systems that update the frame components that are
/// attached to entities, as the `SequenceUpdateEvent`s include the new frame index, which is only
/// guaranteed to be valid for the current dispatcher run.
#[derive(Debug, Default, TypeName, new)]
pub struct SequenceUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceUpdateSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `WaitSequenceHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_handles: ReadStorage<'s, WaitSequenceHandle>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameFreezeClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_freeze_clocks: WriteStorage<'s, FrameFreezeClock>,
    /// `FrameWaitClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Write<'s, EventChannel<SequenceUpdateEvent>>,
}

#[derive(Debug)]
struct SequenceUpdateParams<'p> {
    entity: Entity,
    wait_sequence_handle: &'p WaitSequenceHandle,
    frame_index_clock: &'p mut FrameIndexClock,
    frame_wait_clock: &'p mut FrameWaitClock,
    sequence_status: &'p mut SequenceStatus,
}

impl SequenceUpdateSystem {
    fn start_sequence(
        wait_sequence_assets: &AssetStorage<WaitSequence>,
        SequenceUpdateParams {
            entity: _entity,
            wait_sequence_handle,
            frame_index_clock,
            frame_wait_clock,
            sequence_status,
        }: SequenceUpdateParams,
    ) {
        frame_index_clock.reset();
        frame_wait_clock.reset();

        // Set to ongoing, meaning we must be sure that this is the only system
        // that needs to read the `SequenceStatus::Begin` status.
        *sequence_status = SequenceStatus::Ongoing;

        // Update the frame_index_clock limit because we already hold a mutable
        // borrow of the component storage.
        let wait_sequence = wait_sequence_assets
            .get(wait_sequence_handle)
            .expect("Expected `WaitSequence` to be loaded.");
        (*frame_index_clock).limit = wait_sequence.len();

        Self::update_frame_wait_clock_limit(wait_sequence, frame_wait_clock, 0);
    }

    /// Returns true if the entity is **not frozen**, ticks the clock otherwise.
    fn entity_unfrozen_tick(
        frame_freeze_clocks: &mut WriteStorage<'_, FrameFreezeClock>,
        entity: Entity,
    ) -> bool {
        frame_freeze_clocks
            .get_mut(entity)
            .map(|frame_freeze_clock| {
                if frame_freeze_clock.is_complete() {
                    true
                } else {
                    frame_freeze_clock.tick();
                    false
                }
            })
            .unwrap_or(true)
    }

    fn entity_frame_wait_tick(
        wait_sequence_assets: &AssetStorage<WaitSequence>,
        sequence_update_ec: &mut EventChannel<SequenceUpdateEvent>,
        sequence_update_params: SequenceUpdateParams,
    ) {
        let SequenceUpdateParams {
            entity,
            wait_sequence_handle,
            frame_index_clock,
            frame_wait_clock,
            sequence_status,
        } = sequence_update_params;

        frame_wait_clock.tick();

        if frame_wait_clock.is_complete() {
            // Switch to next frame, or if there is no next frame, switch
            // `SequenceStatus` to `End`.

            frame_index_clock.tick();

            if frame_index_clock.is_complete() {
                *sequence_status = SequenceStatus::End;

                let frame_index = (*frame_index_clock).value.checked_sub(1).unwrap_or(0);
                sequence_update_ec.single_write(SequenceUpdateEvent::SequenceEnd {
                    entity,
                    frame_index,
                });
            } else {
                frame_wait_clock.reset();

                let frame_index = (*frame_index_clock).value;

                // Update limit for `FrameWaitClock`.
                let wait_sequence = wait_sequence_assets
                    .get(wait_sequence_handle)
                    .expect("Expected `WaitSequence` to be loaded.");

                Self::update_frame_wait_clock_limit(wait_sequence, frame_wait_clock, frame_index);

                sequence_update_ec.single_write(SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                });
            }
        }
    }

    fn update_frame_wait_clock_limit(
        wait_sequence: &WaitSequence,
        frame_wait_clock: &mut FrameWaitClock,
        frame_index: usize,
    ) {
        let wait = wait_sequence.get(frame_index).unwrap_or_else(|| {
            panic!(
                "Expected wait sequence to have frame index: `{}`. `WaitSequence`: {:?}",
                frame_index, wait_sequence
            )
        });
        (*frame_wait_clock).limit = **wait as usize;
    }
}

impl<'s> System<'s> for SequenceUpdateSystem {
    type SystemData = SequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        SequenceUpdateSystemData {
            entities,
            wait_sequence_handles,
            wait_sequence_assets,
            mut frame_index_clocks,
            mut frame_freeze_clocks,
            mut frame_wait_clocks,
            mut sequence_statuses,
            mut sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &wait_sequence_handles,
            &mut frame_index_clocks,
            &mut frame_wait_clocks,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    wait_sequence_handle,
                    mut frame_index_clock,
                    mut frame_wait_clock,
                    mut sequence_status,
                )| {
                    let sequence_update_params = SequenceUpdateParams {
                        entity,
                        wait_sequence_handle: &wait_sequence_handle,
                        frame_index_clock: &mut frame_index_clock,
                        frame_wait_clock: &mut frame_wait_clock,
                        sequence_status: &mut sequence_status,
                    };
                    match sequence_update_params.sequence_status {
                        SequenceStatus::Begin => {
                            Self::start_sequence(&wait_sequence_assets, sequence_update_params);
                        }
                        SequenceStatus::Ongoing => {
                            if Self::entity_unfrozen_tick(&mut frame_freeze_clocks, entity) {
                                Self::entity_frame_wait_tick(
                                    &wait_sequence_assets,
                                    &mut sequence_update_ec,
                                    sequence_update_params,
                                );
                            }
                        }
                        SequenceStatus::End => {
                            Self::entity_unfrozen_tick(&mut frame_freeze_clocks, entity);
                        }
                    }
                },
            );
    } // kcov-ignore
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Entities, Entity, Read, ReadExpect, SystemData, World, WriteStorage},
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

    use super::{SequenceUpdateSystem, SequenceUpdateSystemData};

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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(10, 10),
                    frame_wait_clock(10, 10),
                    None,
                    SequenceStatus::Begin,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(0, 2),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(0, 2),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(1, 2),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(1, 2),
                    Some(frame_freeze_clock(1, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(1, 2),
                    Some(frame_freeze_clock(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(0, 2),
                    Some(frame_freeze_clock(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(1, 2),
                    Some(frame_freeze_clock(2, 2)),
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(1, 2),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(1, 5),
                    frame_wait_clock(0, 3),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| {
                let events = frame_begin_events(world, 1);
                expect_events(world, events);
            })
            .run_isolated()
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
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(4, 5),
                    frame_wait_clock(5, 6),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(6, 6),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| {
                let events = sequence_end_events(world, 4);
                expect_events(world, events);
            })
            .run_isolated()
    }

    #[test]
    fn does_not_tick_frame_clocks_when_sequence_end() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(2, 2),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(2, 2),
                    None,
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
    }

    #[test]
    fn ticks_frame_freeze_clock_when_sequence_end() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(2, 2),
                    Some(frame_freeze_clock(0, 2)),
                    SequenceStatus::End,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(2, 2),
                    Some(frame_freeze_clock(1, 2)),
                    SequenceStatus::End,
                )
            })
            .with_assertion(|world| expect_events(world, vec![]))
            .run_isolated()
    }

    fn setup_system_data(world: &mut World) {
        SequenceUpdateSystemData::setup(&mut world.res);
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.add_resource(reader_id);
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

        world.add_resource(entity);
    }

    fn frame_index_clock(value: usize, limit: usize) -> FrameIndexClock {
        let mut frame_index_clock = FrameIndexClock::default();
        (*frame_index_clock).value = value;
        (*frame_index_clock).limit = limit;
        frame_index_clock
    }

    fn frame_wait_clock(value: usize, limit: usize) -> FrameWaitClock {
        let mut frame_wait_clock = FrameWaitClock::default();
        (*frame_wait_clock).value = value;
        (*frame_wait_clock).limit = limit;
        frame_wait_clock
    }

    fn frame_freeze_clock(value: usize, limit: usize) -> FrameFreezeClock {
        let mut frame_freeze_clock = FrameFreezeClock::default();
        (*frame_freeze_clock).value = value;
        (*frame_freeze_clock).limit = limit;
        frame_freeze_clock
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
                SequenceUpdateEvent::SequenceBegin { entity }
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
