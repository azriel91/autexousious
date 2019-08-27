use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::{
    CharacterControlTransitionsHandle, CharacterControlTransitionsSequence,
    CharacterControlTransitionsSequenceHandle,
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{loaded::SequenceId, play::SequenceUpdateEvent};

/// Updates the `CharacterControlTransitionsHandle` when sequence ID changes.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterControlTransitionsUpdateSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterControlTransitionsUpdateSystemData<'s> {
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `CharacterControlTransitionsSequenceHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_cts_handles: ReadStorage<'s, CharacterControlTransitionsSequenceHandle>,
    /// `CharacterControlTransitionsSequence` assets.
    #[derivative(Debug = "ignore")]
    pub character_cts_assets: Read<'s, AssetStorage<CharacterControlTransitionsSequence>>,
    /// `CharacterControlTransitionsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_handles: WriteStorage<'s, CharacterControlTransitionsHandle>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub character_sequence_names: ReadStorage<'s, SequenceId>,
}

impl<'s> System<'s> for CharacterControlTransitionsUpdateSystem {
    type SystemData = CharacterControlTransitionsUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterControlTransitionsUpdateSystemData {
            sequence_update_ec,
            character_cts_handles,
            character_cts_assets,
            mut character_control_transitions_handles,
            character_sequence_names,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id.as_mut().expect(
                    "Expected reader ID to exist for CharacterControlTransitionsUpdateSystem.",
                ),
            )
            // kcov-ignore-start
            .filter(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { .. }
                | SequenceUpdateEvent::FrameBegin { .. } = ev
                {
                    true
                } else {
                    false
                }
            })
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                // `SequenceUpdateEvent`s are also sent for non-object entities such as map layers
                if let Some(character_cts_handle) = character_cts_handles.get(entity) {
                    let character_control_transitions_sequence = character_cts_assets
                        .get(character_cts_handle)
                        .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

                    if frame_index < character_control_transitions_sequence.len() {
                        let character_control_transitions_handle =
                            &character_control_transitions_sequence[frame_index];

                        character_control_transitions_handles
                            .insert(entity, character_control_transitions_handle.clone())
                            .expect("Failed to insert `CharacterControlTransitions` component.");
                    } else {
                        let character_sequence_name = character_sequence_names.get(entity).expect(
                            "Expected entity with `CharacterControlTransitionsSequenceHandle` \
                             to have `SequenceId`.",
                        );

                        error!(
                            "Attempted to access index `{}` for sequence ID: `{:?}`",
                            frame_index, character_sequence_name
                        );
                    }
                }
            });
        // kcov-ignore-end
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Entities, Join, Read, ReadStorage, World, WorldExt, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use character_model::loaded::{
        CharacterControlTransition, CharacterControlTransitions, CharacterControlTransitionsHandle,
        CharacterControlTransitionsSequenceHandle,
    };
    use game_input_model::ControlAction;
    use sequence_model::{
        loaded::{ActionPress, ControlTransition, ControlTransitions, SequenceId},
        play::{FrameIndexClock, SequenceUpdateEvent},
    };

    use super::CharacterControlTransitionsUpdateSystem;

    #[test]
    fn updates_transitions_on_sequence_begin_event() -> Result<(), Error> {
        run_test(
            // First frame in the sequence.
            FrameIndexClock::new_with_value(5, 0),
            sequence_begin_events,
        )
    }

    #[test]
    fn updates_transitions_on_frame_begin_event() -> Result<(), Error> {
        run_test(
            // Third frame in the sequence.
            FrameIndexClock::new_with_value(5, 2),
            frame_begin_events,
        )
    }

    fn run_test(
        frame_index_clock: FrameIndexClock,
        sequence_update_events_fn: fn(&mut World) -> Vec<SequenceUpdateEvent>,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(CharacterControlTransitionsUpdateSystem::new(), "", &[])
            .with_setup(move |world| {
                let character_cts_handle = SequenceQueries::character_cts_handle(
                    world,
                    &CHAR_BAT_SLUG.clone(),
                    SequenceId::new(1),
                );
                initial_values(world, frame_index_clock, character_cts_handle)
            })
            .with_setup(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_transitions(world, transitions()))
            .run_isolated()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock_setup: FrameIndexClock,
        character_cts_handle_initial: CharacterControlTransitionsSequenceHandle,
    ) {
        let (
            _entities,
            _sequence_ids,
            mut frame_index_clocks,
            _character_control_transitions_handles,
            mut character_cts_handles,
        ) = world.system_data::<TestSystemData>();

        (&mut frame_index_clocks, &mut character_cts_handles)
            .join()
            // kcov-ignore-start
            .for_each(|(frame_index_clock, character_cts_handle)| {
                *frame_index_clock = frame_index_clock_setup;

                *character_cts_handle = character_cts_handle_initial.clone();
            });
        // kcov-ignore-end
    }

    fn expect_transitions(
        world: &mut World,
        expected_character_control_transitions: CharacterControlTransitions,
    ) {
        let (
            character_control_transitions_assets,
            character_control_transitions_handles,
            sequence_statuses,
        ) = world.system_data::<(
            Read<AssetStorage<CharacterControlTransitions>>,
            ReadStorage<CharacterControlTransitionsHandle>,
            ReadStorage<SequenceId>,
        )>();

        (&character_control_transitions_handles, &sequence_statuses)
            .join()
            // kcov-ignore-start
            .for_each(|(character_control_transitions_handle, _sequence_status)| {
                let character_control_transitions = character_control_transitions_assets
                    .get(character_control_transitions_handle)
                    .expect("Expected `CharacterControlTransitions` to be loaded.");

                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );
            });
        // kcov-ignore-end
    }

    fn transitions() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition::new(
                ControlTransition::ActionPress(ActionPress::new(
                    ControlAction::Attack,
                    SequenceId::new(1),
                )),
                vec![],
            ),
            CharacterControlTransition::new(
                ControlTransition::ActionPress(ActionPress::new(
                    ControlAction::Jump,
                    SequenceId::new(7),
                )),
                vec![],
            ),
        ]))
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            sequence_ids,
            frame_index_clocks,
            character_control_transitions_handles,
            character_cts_handles,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &sequence_ids,
            &frame_index_clocks,
            &character_control_transitions_handles,
            &character_cts_handles,
        )
            .join()
            // kcov-ignore-start
            .map(
                |(entity, sequence_id, _, _, _)| SequenceUpdateEvent::SequenceBegin {
                    entity,
                    sequence_id: *sequence_id,
                },
            )
            // kcov-ignore-end
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
            _sequence_ids,
            frame_index_clocks,
            character_control_transitions_handles,
            character_cts_handles,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &character_control_transitions_handles,
            &character_cts_handles,
        )
            .join()
            // kcov-ignore-start
            .map(|(entity, frame_index_clock, _, _)| {
                let frame_index = (*frame_index_clock).value;
                SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                }
            })
            // kcov-ignore-end
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        ReadStorage<'s, SequenceId>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, CharacterControlTransitionsHandle>,
        WriteStorage<'s, CharacterControlTransitionsSequenceHandle>,
    );
}
