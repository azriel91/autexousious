use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{
    config::Repeat,
    loaded::{ComponentSequences, ComponentSequencesHandle},
    play::{
        FrameFreezeClock, FrameIndexClock, FrameWaitClock, SequenceStatus, SequenceUpdateEvent,
    },
};
use shred_derive::SystemData;

/// Updates the frame limit clock and logic clock for entities with sequences.
#[derive(Debug, Default, NamedType, new)]
pub struct SequenceUpdateSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SequenceUpdateSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub repeats: ReadStorage<'s, Repeat>,
    /// `ComponentSequencesHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: ReadStorage<'s, ComponentSequencesHandle>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
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
    component_sequences_handle: &'p ComponentSequencesHandle,
    frame_index_clock: &'p mut FrameIndexClock,
    frame_wait_clock: &'p mut FrameWaitClock,
    sequence_status: &'p mut SequenceStatus,
}

impl SequenceUpdateSystem {
    fn start_sequence(
        component_sequences_assets: &AssetStorage<ComponentSequences>,
        sequence_update_ec: &mut EventChannel<SequenceUpdateEvent>,
        SequenceUpdateParams {
            entity,
            component_sequences_handle,
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
        (*frame_index_clock).limit = component_sequences_assets
            .get(component_sequences_handle)
            .expect("Expected component_sequences to be loaded.")
            .frame_count();

        sequence_update_ec.single_write(SequenceUpdateEvent::SequenceBegin { entity });
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
        component_sequences_assets: &AssetStorage<ComponentSequences>,
        mut sequence_update_ec: &mut EventChannel<SequenceUpdateEvent>,
        repeats: &ReadStorage<'_, Repeat>,
        sequence_update_params: SequenceUpdateParams,
    ) {
        let SequenceUpdateParams {
            entity,
            component_sequences_handle,
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

                if repeats.contains(entity) {
                    let sequence_update_params = SequenceUpdateParams {
                        entity,
                        component_sequences_handle,
                        frame_index_clock,
                        frame_wait_clock,
                        sequence_status,
                    };

                    Self::start_sequence(
                        &component_sequences_assets,
                        &mut sequence_update_ec,
                        sequence_update_params,
                    );
                }
            } else {
                frame_wait_clock.reset();

                let frame_index = (*frame_index_clock).value;
                sequence_update_ec.single_write(SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                });
            }
        }
    }
}

impl<'s> System<'s> for SequenceUpdateSystem {
    type SystemData = SequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        SequenceUpdateSystemData {
            entities,
            repeats,
            component_sequences_handles,
            component_sequences_assets,
            mut frame_index_clocks,
            mut frame_freeze_clocks,
            mut frame_wait_clocks,
            mut sequence_statuses,
            mut sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &component_sequences_handles,
            &mut frame_index_clocks,
            &mut frame_wait_clocks,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    component_sequences_handle,
                    mut frame_index_clock,
                    mut frame_wait_clock,
                    mut sequence_status,
                )| {
                    let sequence_update_params = SequenceUpdateParams {
                        entity,
                        component_sequences_handle: &component_sequences_handle,
                        frame_index_clock: &mut frame_index_clock,
                        frame_wait_clock: &mut frame_wait_clock,
                        sequence_status: &mut sequence_status,
                    };
                    match sequence_update_params.sequence_status {
                        SequenceStatus::Begin => {
                            Self::start_sequence(
                                &component_sequences_assets,
                                &mut sequence_update_ec,
                                sequence_update_params,
                            );
                        }
                        SequenceStatus::Ongoing => {
                            if Self::entity_unfrozen_tick(&mut frame_freeze_clocks, entity) {
                                Self::entity_frame_wait_tick(
                                    &component_sequences_assets,
                                    &mut sequence_update_ec,
                                    &repeats,
                                    sequence_update_params,
                                );
                            }
                        }
                        SequenceStatus::End => {} // do nothing
                    }
                },
            );
    } // kcov-ignore
}

#[cfg(test)]
mod tests {
    use amethyst::{
        ecs::{Entities, Entity, SystemData, World, WriteStorage},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::config::CharacterSequenceId;
    use logic_clock::LogicClock;
    use sequence_model::{
        config::Repeat,
        loaded::ComponentSequencesHandle,
        play::{
            FrameFreezeClock, FrameIndexClock, FrameWaitClock, SequenceStatus, SequenceUpdateEvent,
        },
    };

    use super::{SequenceUpdateSystem, SequenceUpdateSystemData};

    /// Asserts the following:
    ///
    /// * Resets `FrameIndexClock`.
    /// * Updates the `FrameIndexClock` limit to the new sequence's limit.
    /// * Resets `LogicClock` (frame wait counter).
    /// * `SequenceUpdateEvent::SequenceBegin` events are sent.
    #[test]
    fn resets_frame_wait_clocks_and_sends_event_on_sequence_begin() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(10, 10),
                    frame_wait_clock(10, 10),
                    None,
                    SequenceStatus::Begin,
                    false,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(0, 5),
                    frame_wait_clock(0, 10),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| {
                let events = sequence_begin_events(world);
                expect_events(world, events);
            })
            .run()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameWaitClock`.
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
                    false,
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
            .run()
    }

    /// Asserts the following when a frame is still in progress but entity is frozen:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameFreezeClock`.
    /// * No change to `LogicClock`.
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
                    false,
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
            .run()
    }

    /// Asserts the following when a frame is still in progress but entity is frozen:
    ///
    /// * No change to `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * No change to `FrameFreezeClock`.
    /// * Ticks `FrameWaitClock`.
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
                    false,
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
            .run()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Resets `LogicClock` (frame wait counter).
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
                    false,
                )
            })
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| {
                expect_values(
                    world,
                    frame_index_clock(1, 5),
                    frame_wait_clock(0, 2),
                    None,
                    SequenceStatus::Ongoing,
                )
            })
            .with_assertion(|world| {
                let events = frame_begin_events(world, 1);
                expect_events(world, events);
            })
            .run()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `FrameWaitClock` value.
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
                    frame_wait_clock(1, 2),
                    None,
                    SequenceStatus::Ongoing,
                    false,
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
            .with_assertion(|world| {
                let events = sequence_end_events(world, 4);
                expect_events(world, events);
            })
            .run()
    }

    /// Asserts the following when the entity has the `Repeat` component:
    ///
    /// * Resets `FrameIndexClock`.
    /// * Updates the `FrameIndexClock` limit to the new sequence's limit.
    /// * Resets `LogicClock` (frame wait counter).
    /// * `SequenceEnd` and `SequenceBegin` events are both sent.
    /// * Sets `SequenceStatus` to `SequenceStatus::Ongoing`.
    #[test]
    fn sends_events_when_frame_ends_and_sequence_ends_and_repeat() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(4, 5),
                    frame_wait_clock(1, 2),
                    None,
                    SequenceStatus::Ongoing,
                    true,
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
            .with_assertion(|world| {
                let events = sequence_end_and_begin_events(world, 4);
                expect_events(world, events);
            })
            .run()
    }

    #[test]
    fn does_nothing_when_sequence_end() -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    frame_index_clock(5, 5),
                    frame_wait_clock(2, 2),
                    None,
                    SequenceStatus::End,
                    false,
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
            .run()
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
        repeat: bool,
    ) {
        let run_stop_handle = SequenceQueries::component_sequences_handle(
            world,
            &ASSETS_CHAR_BAT_SLUG.clone(),
            CharacterSequenceId::RunStop,
        );

        let entity = {
            let (
                entities,
                mut frame_index_clocks,
                mut frame_wait_clocks,
                mut frame_freeze_clocks,
                mut component_sequences_handles,
                mut sequence_statuses,
            ) = world.system_data::<TestSystemData>();
            let mut repeats = world.write_storage::<Repeat>();

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
            component_sequences_handles
                .insert(entity, run_stop_handle)
                .expect("Failed to insert run_stop_handle component.");
            sequence_statuses
                .insert(entity, sequence_status)
                .expect("Failed to insert sequence_status component.");
            if repeat {
                repeats
                    .insert(entity, Repeat)
                    .expect("Failed to insert `Repeat` component.");
            }

            entity
        };

        world.add_resource(entity);
    }

    fn frame_index_clock(value: usize, limit: usize) -> FrameIndexClock {
        let mut frame_index_clock = FrameIndexClock::new(LogicClock::default());
        (*frame_index_clock).value = value;
        (*frame_index_clock).limit = limit;
        frame_index_clock
    }

    fn frame_wait_clock(value: usize, limit: usize) -> FrameWaitClock {
        let mut frame_wait_clock = FrameWaitClock::new(LogicClock::default());
        (*frame_wait_clock).value = value;
        (*frame_wait_clock).limit = limit;
        frame_wait_clock
    }

    fn frame_freeze_clock(value: usize, limit: usize) -> FrameFreezeClock {
        let mut frame_freeze_clock = FrameFreezeClock::new(LogicClock::default());
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

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin { entity }]
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

    fn sequence_end_and_begin_events(
        world: &mut World,
        frame_index: usize,
    ) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();

        vec![
            SequenceUpdateEvent::SequenceEnd {
                entity,
                frame_index,
            },
            SequenceUpdateEvent::SequenceBegin { entity },
        ]
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, FrameWaitClock>,
        WriteStorage<'s, FrameFreezeClock>,
        WriteStorage<'s, ComponentSequencesHandle>,
        WriteStorage<'s, SequenceStatus>,
    );
}
