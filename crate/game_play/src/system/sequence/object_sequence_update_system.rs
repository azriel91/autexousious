use std::marker::PhantomData;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use logic_clock::LogicClock;
use named_type::NamedType;
use named_type_derive::NamedType;
use object_model::{
    entity::{FrameIndexClock, SequenceStatus},
    loaded::{GameObject, ObjectWrapper},
};
use shred_derive::SystemData;

use crate::ObjectSequenceUpdateEvent;

/// Updates the logic clock and sequence ID for objects.
///
/// # Type Parameters
///
/// * `O`: `GameObject` type, e.g. `Character`.
#[derive(Debug, Default, NamedType, new)]
pub struct ObjectSequenceUpdateSystem<O> {
    /// PhantomData.
    phantom_data: PhantomData<O>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSequenceUpdateSystemData<'s, O>
where
    O: GameObject,
{
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `Handle<O::ObjectWrapper>` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: ReadStorage<'s, Handle<O::ObjectWrapper>>,
    /// `O::ObjectWrapper` assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `LogicClock` component storage.
    #[derivative(Debug = "ignore")]
    pub logic_clocks: WriteStorage<'s, LogicClock>,
    /// `O::SequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, O::SequenceId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// Event channel for `ObjectSequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub object_sequence_update_ec: Write<'s, EventChannel<ObjectSequenceUpdateEvent>>,
}

impl<O> ObjectSequenceUpdateSystem<O>
where
    O: GameObject,
{
    fn sequence_frame_count(
        object_assets: &AssetStorage<O::ObjectWrapper>,
        object_handle: &Handle<O::ObjectWrapper>,
        sequence_id: O::SequenceId,
    ) -> usize {
        let object = object_assets
            .get(object_handle)
            .expect("Expected object to be loaded.");
        let component_sequences = object
            .inner()
            .component_sequences
            .get(&sequence_id)
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get `ComponentSequences` for sequence ID: \
                     `{:?}`.",
                    sequence_id
                );
            });

        component_sequences.frame_count()
    }
}

impl<'s, O> System<'s> for ObjectSequenceUpdateSystem<O>
where
    O: GameObject,
{
    type SystemData = ObjectSequenceUpdateSystemData<'s, O>;

    fn run(
        &mut self,
        ObjectSequenceUpdateSystemData {
            entities,
            object_handles,
            object_assets,
            mut frame_index_clocks,
            mut logic_clocks,
            mut sequence_ids,
            mut sequence_statuses,
            mut object_sequence_update_ec,
        }: Self::SystemData,
    ) {
        (
            &entities,
            &object_handles,
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut sequence_ids,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(
                    entity,
                    object_handle,
                    frame_index_clock,
                    logic_clock,
                    sequence_id,
                    sequence_status,
                )| {
                    match sequence_status {
                        SequenceStatus::Begin => {
                            // Retrieve frame indicies separately as we use a `FlaggedStorage` to
                            // track if it has been changed, to update frame components.
                            frame_index_clock.reset();
                            logic_clock.reset();

                            // Set to ongoing, meaning we must be sure that this is the only system
                            // that needs to read the `SequenceStatus::Begin` status.
                            *sequence_status = SequenceStatus::Ongoing;

                            // Update the frame_index_clock limit because we already hold a mutable
                            // borrow of the component storage.
                            (*frame_index_clock).limit = Self::sequence_frame_count(
                                &object_assets,
                                &object_handle,
                                *sequence_id,
                            );

                            object_sequence_update_ec
                                .single_write(ObjectSequenceUpdateEvent::SequenceBegin { entity });
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
                                    object_sequence_update_ec.single_write(
                                        ObjectSequenceUpdateEvent::FrameBegin { entity },
                                    );
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
        ecs::{Entities, Join, SystemData, World, WriteStorage},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_model::{config::CharacterSequenceId, loaded::Character};
    use logic_clock::LogicClock;
    use object_model::entity::{FrameIndexClock, SequenceStatus};

    use super::{ObjectSequenceUpdateSystem, ObjectSequenceUpdateSystemData};
    use crate::ObjectSequenceUpdateEvent;

    /// Asserts the following:
    ///
    /// * Resets `FrameIndexClock`.
    /// * Updates the `FrameIndexClock` limit to the new sequence's limit.
    /// * Resets `LogicClock` (frame wait counter).
    /// * `ObjectSequenceUpdateEvent::SequenceBegin` events are sent.
    #[test]
    fn resets_logic_clocks_and_sends_event_on_sequence_begin() -> Result<(), Error> {
        let test_name = "resets_logic_clocks_and_sends_event_on_sequence_begin";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    10,
                    10,
                    10,
                    10,
                    CharacterSequenceId::RunStop,
                    SequenceStatus::Begin,
                )
            })
            .with_system_single(ObjectSequenceUpdateSystem::<Character>::new(), "", &[])
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
    /// * No `ObjectSequenceUpdateEvent`s are sent.
    #[test]
    fn ticks_logic_clock_when_sequence_ongoing() -> Result<(), Error> {
        let test_name = "ticks_logic_clock_when_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    0,
                    5,
                    0,
                    2,
                    CharacterSequenceId::RunStop,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(ObjectSequenceUpdateSystem::<Character>::new(), "", &[])
            .with_assertion(|world| expect_values(world, 0, 5, 1, SequenceStatus::Ongoing))
            .with_assertion(|world| expect_events(world, vec![]))
            .run()
    }

    /// Asserts the following when a frame is still in progress:
    ///
    /// * Ticks `FrameIndexClock` value.
    /// * No change to `FrameIndexClock` limit.
    /// * Resets `LogicClock` (frame wait counter).
    /// * `ObjectSequenceUpdateEvent::FrameBegin` events are sent.
    #[test]
    fn resets_logic_clock_and_sends_event_when_frame_ends_and_sequence_ongoing() -> Result<(), Error>
    {
        let test_name = "resets_logic_clock_and_sends_event_when_frame_ends_and_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    0,
                    5,
                    1,
                    2,
                    CharacterSequenceId::RunStop,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(ObjectSequenceUpdateSystem::<Character>::new(), "", &[])
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
    /// * No `ObjectSequenceUpdateEvent`s are sent.
    /// * Sets `SequenceStatus` to `SequenceStatus::End`.
    #[test]
    fn resets_logic_clock_and_sequence_end_when_frame_ends_and_sequence_ongoing(
    ) -> Result<(), Error> {
        let test_name = "resets_logic_clock_and_sequence_end_when_frame_ends_and_sequence_ongoing";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    4,
                    5,
                    1,
                    2,
                    CharacterSequenceId::RunStop,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(ObjectSequenceUpdateSystem::<Character>::new(), "", &[])
            .with_assertion(|world| expect_values(world, 5, 5, 2, SequenceStatus::End))
            .with_assertion(|world| expect_events(world, vec![]))
            .run()
    }

    #[test]
    fn does_nothing_when_sequence_end() -> Result<(), Error> {
        let test_name = "does_nothing_when_sequence_end";
        AutexousiousApplication::game_base(test_name, false)
            .with_setup(setup_system_data)
            .with_setup(|world| {
                initial_values(
                    world,
                    5,
                    5,
                    2,
                    2,
                    CharacterSequenceId::RunStop,
                    SequenceStatus::Ongoing,
                )
            })
            .with_system_single(ObjectSequenceUpdateSystem::<Character>::new(), "", &[])
            .with_assertion(|world| expect_values(world, 5, 5, 2, SequenceStatus::End))
            .with_assertion(|world| expect_events(world, vec![]))
            .run()
    }

    fn setup_system_data(world: &mut World) {
        ObjectSequenceUpdateSystemData::<Character>::setup(&mut world.res);
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<ObjectSequenceUpdateEvent>>();
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
        sequence_id_initial: CharacterSequenceId,
        sequence_status_initial: SequenceStatus,
    ) {
        let (
            _entities,
            mut frame_index_clocks,
            mut logic_clocks,
            mut sequence_ids,
            mut sequence_statuses,
        ) = world.system_data::<TestSystemData>();

        (
            &mut frame_index_clocks,
            &mut logic_clocks,
            &mut sequence_ids,
            &mut sequence_statuses,
        )
            .join()
            .for_each(
                |(frame_index_clock, logic_clock, sequence_id, sequence_status)| {
                    (*frame_index_clock).value = frame_index_clock_value;
                    (*frame_index_clock).limit = frame_index_clock_limit;

                    (*logic_clock).value = logic_clock_value;
                    (*logic_clock).limit = logic_clock_limit;

                    *sequence_id = sequence_id_initial;
                    *sequence_status = sequence_status_initial;
                },
            );
    }

    fn expect_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        logic_clock_value: usize,
        sequence_status_expected: SequenceStatus,
    ) {
        let (_entities, frame_index_clocks, logic_clocks, _sequence_ids, sequence_statuses) =
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

    fn expect_events(world: &mut World, expect_events: Vec<ObjectSequenceUpdateEvent>) {
        let mut reader_id = world.write_resource::<ReaderId<ObjectSequenceUpdateEvent>>();
        let ec = world.read_resource::<EventChannel<ObjectSequenceUpdateEvent>>();

        // Map owned values into references.
        let expect_events = expect_events.iter().collect::<Vec<_>>();
        assert_eq!(expect_events, ec.read(&mut reader_id).collect::<Vec<_>>())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<ObjectSequenceUpdateEvent> {
        let (entities, frame_index_clocks, logic_clocks, _sequence_ids, sequence_statuses) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &sequence_statuses,
        )
            .join()
            .map(|(entity, _, _, _)| ObjectSequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<ObjectSequenceUpdateEvent> {
        let (entities, frame_index_clocks, logic_clocks, _sequence_ids, sequence_statuses) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &logic_clocks,
            &sequence_statuses,
        )
            .join()
            .map(|(entity, _, _, _)| ObjectSequenceUpdateEvent::FrameBegin { entity })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, LogicClock>,
        WriteStorage<'s, CharacterSequenceId>,
        WriteStorage<'s, SequenceStatus>,
    );
}
