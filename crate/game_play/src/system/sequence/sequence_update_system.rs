use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{
    entity::{FrameIndexClock, SequenceStatus},
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
    pub component_sequences_handles: ReadStorage<'s, ComponentSequencesHandle>,
    /// `ComponentSequences` assets.
    #[derivative(Debug = "ignore")]
    pub component_sequences_assets: Read<'s, AssetStorage<ComponentSequences>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Write<'s, EventChannel<SequenceUpdateEvent>>,
}

impl<'s> System<'s> for SequenceUpdateSystem {
    type SystemData = SequenceUpdateSystemData<'s>;

    fn run(
        &mut self,
        SequenceUpdateSystemData {
            entities,
            component_sequences_handles,
            component_sequences_assets,
            mut frame_index_clocks,
            mut logic_clocks,
            mut sequence_statuses,
            mut sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &component_sequences_handles,
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    component_sequences_handle,
                    frame_index_clock,
                    logic_clock,
                    sequence_status,
                )| {
                    match sequence_status {
                        SequenceStatus::Begin => {
                            frame_index_clock.reset();
                            logic_clock.reset();

                            // Set to ongoing, meaning we must be sure that this is the only system
                            // that needs to read the `SequenceStatus::Begin` status.
                            *sequence_status = SequenceStatus::Ongoing;

                            // Update the frame_index_clock limit because we already hold a mutable
                            // borrow of the component storage.
                            (*frame_index_clock).limit = component_sequences_assets
                                .get(component_sequences_handle)
                                .expect("Expected component_sequences to be loaded.")
                                .frame_count();

                            sequence_update_ec
                                .single_write(SequenceUpdateEvent::SequenceBegin { entity });
                        }
                        SequenceStatus::Ongoing => {
                            logic_clock.tick();

                            if logic_clock.is_complete() {
                                // Switch to next frame, or if there is no next frame, switch
                                // `SequenceStatus` to `End`.

                                frame_index_clock.tick();

                                if frame_index_clock.is_complete() {
                                    *sequence_status = SequenceStatus::End;
                                } else {
                                    logic_clock.reset();
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
        assets::{AssetStorage, Prefab},
        ecs::{Entities, Join, SystemData, World, WriteStorage},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::loaded::SlugAndHandle;
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_loading::CharacterPrefab;
    use character_model::{config::CharacterSequenceId, loaded::CharacterObjectWrapper};
    use logic_clock::LogicClock;
    use object_loading::ObjectPrefab;
    use object_model::loaded::ObjectWrapper;
    use sequence_model::{
        entity::{FrameIndexClock, SequenceStatus},
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
    fn resets_logic_clocks_and_sends_event_on_sequence_begin() -> Result<(), Error> {
        let test_name = "resets_logic_clocks_and_sends_event_on_sequence_begin";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 10, 10, 10, 10, SequenceStatus::Begin))
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
    fn ticks_logic_clock_when_sequence_ongoing() -> Result<(), Error> {
        let test_name = "ticks_logic_clock_when_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 0, 5, 0, 2, SequenceStatus::Ongoing))
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
    fn resets_logic_clock_and_sends_event_when_frame_ends_and_sequence_ongoing() -> Result<(), Error>
    {
        let test_name = "resets_logic_clock_and_sends_event_when_frame_ends_and_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 0, 5, 1, 2, SequenceStatus::Ongoing))
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
    /// * No `SequenceUpdateEvent`s are sent.
    /// * Sets `SequenceStatus` to `SequenceStatus::End`.
    #[test]
    fn resets_logic_clock_and_sequence_end_when_frame_ends_and_sequence_ongoing(
    ) -> Result<(), Error> {
        let test_name = "resets_logic_clock_and_sequence_end_when_frame_ends_and_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 4, 5, 1, 2, SequenceStatus::Ongoing))
            .with_system_single(SequenceUpdateSystem::new(), "", &[])
            .with_assertion(|world| expect_values(world, 5, 5, 2, SequenceStatus::End))
            .with_assertion(|world| expect_events(world, vec![]))
            .run()
    }

    #[test]
    fn does_nothing_when_sequence_end() -> Result<(), Error> {
        let test_name = "does_nothing_when_sequence_end";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| initial_values(world, 5, 5, 2, 2, SequenceStatus::Ongoing))
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
        logic_clock_value: usize,
        logic_clock_limit: usize,
        sequence_status_initial: SequenceStatus,
    ) {
        let run_stop_handle = component_sequences_handle(world, CharacterSequenceId::RunStop);

        let (
            _entities,
            mut frame_index_clocks,
            mut logic_clocks,
            mut component_sequences_handles,
            mut sequence_statuses,
        ) = world.system_data::<TestSystemData>();

        (
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut component_sequences_handles,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(frame_index_clock, logic_clock, component_sequences_handle, sequence_status)| {
                    (*frame_index_clock).value = frame_index_clock_value;
                    (*frame_index_clock).limit = frame_index_clock_limit;

                    (*logic_clock).value = logic_clock_value;
                    (*logic_clock).limit = logic_clock_limit;

                    *component_sequences_handle = run_stop_handle.clone();
                    *sequence_status = sequence_status_initial;
                },
            );
    }

    // Quite unergonomic =/
    fn component_sequences_handle(
        world: &mut World,
        sequence_id: CharacterSequenceId,
    ) -> ComponentSequencesHandle {
        let snh = SlugAndHandle::from((&*world, ASSETS_CHAR_BAT_SLUG.clone()));
        let character_prefab_assets =
            world.read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();
        let bat_char_prefab = character_prefab_assets
            .get(&snh.handle)
            .expect("Expected bat character prefab to be loaded.");
        let object_wrapper_handle = {
            let object_prefab = &bat_char_prefab
                .entities()
                .next()
                .expect("Expected bat character main entity to exist.")
                .data()
                .expect("Expected bat character prefab to contain data.")
                .object_prefab;
            if let ObjectPrefab::Handle(handle) = object_prefab {
                handle.clone()
            } else {
                panic!("Expected bat object prefab to be loaded.")
            }
        };

        let object_wrapper_assets = world.read_resource::<AssetStorage<CharacterObjectWrapper>>();
        let object_wrapper = object_wrapper_assets
            .get(&object_wrapper_handle)
            .expect("Expected bat object wrapper to be loaded.");
        object_wrapper
            .inner()
            .component_sequences_handles
            .get(&sequence_id)
            .expect("Expected `RunStop` sequence to exist.")
            .clone()
    }

    fn expect_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        logic_clock_value: usize,
        sequence_status_expected: SequenceStatus,
    ) {
        let (_entities, frame_index_clocks, logic_clocks, _, sequence_statuses) =
            world.system_data::<TestSystemData>();

        (&frame_index_clocks, &logic_clocks, &sequence_statuses)
            .join()
            .for_each(|(frame_index_clock, logic_clock, sequence_status)| {
                assert_eq!(frame_index_clock_value, (*frame_index_clock).value);
                assert_eq!(frame_index_clock_limit, (*frame_index_clock).limit);
                assert_eq!(logic_clock_value, (*logic_clock).value);
                assert_eq!(sequence_status_expected, *sequence_status);
            });
    }

    fn expect_events(world: &mut World, expect_events: Vec<SequenceUpdateEvent>) {
        let mut reader_id = world.write_resource::<ReaderId<SequenceUpdateEvent>>();
        let ec = world.read_resource::<EventChannel<SequenceUpdateEvent>>();

        // Map owned values into references.
        let expect_events = expect_events.iter().collect::<Vec<_>>();
        assert_eq!(expect_events, ec.read(&mut reader_id).collect::<Vec<_>>())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (entities, frame_index_clocks, logic_clocks, _, sequence_statuses) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &sequence_statuses,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (entities, frame_index_clocks, logic_clocks, _, sequence_statuses) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &sequence_statuses,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::FrameBegin { entity })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, LogicClock>,
        WriteStorage<'s, ComponentSequencesHandle>,
        WriteStorage<'s, SequenceStatus>,
    );
}
