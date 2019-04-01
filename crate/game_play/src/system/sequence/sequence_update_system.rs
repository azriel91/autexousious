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
    entity::{FrameIndexClock, FrameWaitClock, SequenceStatus},
    loaded::{ComponentSequences, ComponentSequencesHandle},
};
use shred_derive::SystemData;

use crate::SequenceUpdateEvent;

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

impl SequenceUpdateSystem {
    fn start_sequence(
        component_sequences_assets: &AssetStorage<ComponentSequences>,
        component_sequences_handle: &ComponentSequencesHandle,
        sequence_update_ec: &mut EventChannel<SequenceUpdateEvent>,
        entity: Entity,
        frame_index_clock: &mut FrameIndexClock,
        frame_wait_clock: &mut FrameWaitClock,
        sequence_status: &mut SequenceStatus,
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
                    match sequence_status {
                        SequenceStatus::Begin => {
                            Self::start_sequence(
                                &component_sequences_assets,
                                &component_sequences_handle,
                                &mut sequence_update_ec,
                                entity,
                                &mut frame_index_clock,
                                &mut frame_wait_clock,
                                &mut sequence_status,
                            );
                        }
                        SequenceStatus::Ongoing => {
                            frame_wait_clock.tick();

                            if frame_wait_clock.is_complete() {
                                // Switch to next frame, or if there is no next frame, switch
                                // `SequenceStatus` to `End`.

                                frame_index_clock.tick();

                                if frame_index_clock.is_complete() {
                                    *sequence_status = SequenceStatus::End;

                                    sequence_update_ec
                                        .single_write(SequenceUpdateEvent::SequenceEnd { entity });

                                    if repeats.contains(entity) {
                                        Self::start_sequence(
                                            &component_sequences_assets,
                                            &component_sequences_handle,
                                            &mut sequence_update_ec,
                                            entity,
                                            &mut frame_index_clock,
                                            &mut frame_wait_clock,
                                            &mut sequence_status,
                                        );
                                    }
                                } else {
                                    frame_wait_clock.reset();
                                    sequence_update_ec
                                        .single_write(SequenceUpdateEvent::FrameBegin { entity });
                                }
                            }
                        }
                        SequenceStatus::End => {} // do nothing
                    }
                },
            );
    }
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
        entity::{FrameIndexClock, FrameWaitClock, SequenceStatus},
        loaded::ComponentSequencesHandle,
    };

    use super::{SequenceUpdateSystem, SequenceUpdateSystemData};
    use crate::SequenceUpdateEvent;

    /// Asserts the following:
    ///
    /// * Resets `FrameIndexClock`.
    /// * Updates the `FrameIndexClock` limit to the new sequence's limit.
    /// * Resets `LogicClock` (frame wait counter).
    /// * `SequenceUpdateEvent::SequenceBegin` events are sent.
    #[test]
    fn resets_frame_wait_clocks_and_sends_event_on_sequence_begin() -> Result<(), Error> {
        let test_name = "resets_frame_wait_clocks_and_sends_event_on_sequence_begin";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 10, 10, 10, 10, SequenceStatus::Begin, false))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 0, 5, 0, SequenceStatus::Ongoing))
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
    /// * Ticks `LogicClock`.
    /// * No `SequenceUpdateEvent`s are sent.
    #[test]
    fn ticks_frame_wait_clock_when_sequence_ongoing() -> Result<(), Error> {
        let test_name = "ticks_frame_wait_clock_when_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 0, 5, 0, 2, SequenceStatus::Ongoing, false))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 0, 5, 1, SequenceStatus::Ongoing))
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
        let test_name =
            "resets_frame_wait_clock_and_sends_event_when_frame_ends_and_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 0, 5, 1, 2, SequenceStatus::Ongoing, false))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 1, 5, 0, SequenceStatus::Ongoing))
            .with_assertion(|world| {
                let events = frame_begin_events(world);
                expect_events(world, events);
            })
            .run()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Ticks `LogicClock` value.
    /// * `SequenceUpdateEvent::SequenceEnd` event is sent.
    /// * Sets `SequenceStatus` to `SequenceStatus::End`.
    #[test]
    fn sends_end_event_when_frame_ends_and_sequence_ends() -> Result<(), Error> {
        let test_name = "sends_end_event_when_frame_ends_and_sequence_ends";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 4, 5, 1, 2, SequenceStatus::Ongoing, false))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 5, 5, 2, SequenceStatus::End))
            .with_assertion(|world| {
                let events = sequence_end_events(world);
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
        let test_name = "sends_events_when_frame_ends_and_sequence_ends_and_repeat";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 4, 5, 1, 2, SequenceStatus::Ongoing, true))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 0, 5, 0, SequenceStatus::Ongoing))
            .with_assertion(|world| {
                let events = sequence_end_and_begin_events(world);
                expect_events(world, events);
            })
            .run()
    }

    #[test]
    fn does_nothing_when_sequence_end() -> Result<(), Error> {
        let test_name = "does_nothing_when_sequence_end";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 5, 5, 2, 2, SequenceStatus::End, false))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 5, 5, 2, SequenceStatus::End))
            .with_assertion(|world| expect_events(world, vec![]))
            .run()
    }

    fn setup_system_data(world: &mut World) {
        SequenceUpdateSystemData::setup(&mut world.res);
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
            ec.register_reader()
        };
        world.add_resource(reader_id);
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        frame_wait_clock_value: usize,
        frame_wait_clock_limit: usize,
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
                mut component_sequences_handles,
                mut sequence_statuses,
            ) = world.system_data::<TestSystemData>();
            let mut repeats = world.write_storage::<Repeat>();

            let entity = entities.create();

            let mut frame_index_clock = FrameIndexClock::default();
            (*frame_index_clock).value = frame_index_clock_value;
            (*frame_index_clock).limit = frame_index_clock_limit;

            let mut frame_wait_clock = FrameWaitClock::new(LogicClock::default());
            (*frame_wait_clock).value = frame_wait_clock_value;
            (*frame_wait_clock).limit = frame_wait_clock_limit;

            frame_index_clocks
                .insert(entity, frame_index_clock)
                .expect("Failed to insert frame_index_clock component.");
            frame_wait_clocks
                .insert(entity, frame_wait_clock)
                .expect("Failed to insert frame_wait_clock component.");
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

    fn expect_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        frame_wait_clock_value: usize,
        sequence_status_expected: SequenceStatus,
    ) {
        let (_entities, frame_index_clocks, frame_wait_clocks, _, sequence_statuses) =
            world.system_data::<TestSystemData>();

        let entity = *world.read_resource::<Entity>();

        let frame_index_clock = frame_index_clocks
            .get(entity)
            .expect("Expected entity to have frame_index_clock component.");
        let frame_wait_clock = frame_wait_clocks
            .get(entity)
            .expect("Expected entity to have frame_wait_clock component.");
        let sequence_status = sequence_statuses
            .get(entity)
            .expect("Expected entity to have sequence_status component.");

        assert_eq!(frame_index_clock_value, (*frame_index_clock).value);
        assert_eq!(frame_index_clock_limit, (*frame_index_clock).limit);
        assert_eq!(frame_wait_clock_value, (*frame_wait_clock).value);
        assert_eq!(sequence_status_expected, *sequence_status);
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
                | SequenceUpdateEvent::FrameBegin { entity }
                | SequenceUpdateEvent::SequenceEnd { entity } => target_entity == *entity,
            })
            .collect::<Vec<_>>();

        assert_eq!(expect_events, actual_events)
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceBegin { entity }]
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::FrameBegin { entity }]
    }

    fn sequence_end_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();
        vec![SequenceUpdateEvent::SequenceEnd { entity }]
    }

    fn sequence_end_and_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let entity = *world.read_resource::<Entity>();

        vec![
            SequenceUpdateEvent::SequenceEnd { entity },
            SequenceUpdateEvent::SequenceBegin { entity },
        ]
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, FrameWaitClock>,
        WriteStorage<'s, ComponentSequencesHandle>,
        WriteStorage<'s, SequenceStatus>,
    );
}
