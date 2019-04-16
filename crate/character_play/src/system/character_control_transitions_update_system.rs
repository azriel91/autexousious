use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::{
    CharacterControlTransitionsHandle, CharacterControlTransitionsSequence,
    CharacterControlTransitionsSequenceHandle,
};
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::play::SequenceUpdateEvent;
use shred_derive::SystemData;

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
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id.as_mut().expect(
                    "Expected reader ID to exist for CharacterControlTransitionsUpdateSystem.",
                ),
            )
            .for_each(|ev| {
                let entity = ev.entity();
                let frame_index = ev.frame_index();

                // `SequenceUpdateEvent`s are also sent for non-object entities such as map layers
                if let Some(character_cts_handle) = character_cts_handles.get(entity) {
                    let character_control_transitions_sequence = character_cts_assets
                        .get(character_cts_handle)
                        .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

                    let character_control_transitions_handle =
                        &character_control_transitions_sequence[frame_index];

                    character_control_transitions_handles
                        .insert(entity, character_control_transitions_handle.clone())
                        .expect("Failed to insert `CharacterControlTransitions` component.");
                }
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(
            res.fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::AssetStorage,
        ecs::{Entities, Join, Read, ReadStorage, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::{
        config::CharacterSequenceId,
        loaded::{
            CharacterControlTransition, CharacterControlTransitions,
            CharacterControlTransitionsHandle, CharacterControlTransitionsSequenceHandle,
        },
    };
    use game_input_model::ControlAction;
    use sequence_model::{
        loaded::{ControlTransition, ControlTransitionPress, ControlTransitions},
        play::{FrameIndexClock, SequenceUpdateEvent},
    };

    use super::CharacterControlTransitionsUpdateSystem;

    #[test]
    fn updates_transitions_on_sequence_begin_event() -> Result<(), Error> {
        let test_name = "updates_transitions_on_sequence_begin_event";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(CharacterControlTransitionsUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let character_cts_handle = SequenceQueries::character_cts_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack,
                );
                initial_values(
                    world,
                    // first frame in the sequence
                    0,
                    5,
                    character_cts_handle,
                )
            })
            .with_setup(|world| {
                let events = sequence_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_transitions(world, transitions()))
            .run()
    }

    #[test]
    fn updates_transitions_on_frame_begin_event() -> Result<(), Error> {
        let test_name = "updates_transitions_on_frame_begin_event";
        AutexousiousApplication::game_base(test_name, false)
            .with_system(CharacterControlTransitionsUpdateSystem::new(), "", &[])
            .with_setup(|world| {
                let character_cts_handle = SequenceQueries::character_cts_handle(
                    world,
                    &ASSETS_CHAR_BAT_SLUG.clone(),
                    CharacterSequenceId::StandAttack,
                );
                initial_values(
                    world,
                    2, // third frame in the sequence
                    5,
                    character_cts_handle,
                )
            })
            .with_setup(|world| {
                let events = frame_begin_events(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_transitions(world, transitions()))
            .run()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock_value: usize,
        frame_index_clock_limit: usize,
        character_cts_handle_initial: CharacterControlTransitionsSequenceHandle,
    ) {
        let (
            _entities,
            mut frame_index_clocks,
            _character_control_transitions_handles,
            mut character_cts_handles,
            ..
        ) = world.system_data::<TestSystemData>();

        (&mut frame_index_clocks, &mut character_cts_handles)
            .join()
            .for_each(|(frame_index_clock, character_cts_handle)| {
                (*frame_index_clock).value = frame_index_clock_value;
                (*frame_index_clock).limit = frame_index_clock_limit;

                *character_cts_handle = character_cts_handle_initial.clone();
            });
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
            ReadStorage<CharacterSequenceId>,
        )>();

        (&character_control_transitions_handles, &sequence_statuses)
            .join()
            .for_each(|(character_control_transitions_handle, _sequence_status)| {
                let character_control_transitions = character_control_transitions_assets
                    .get(character_control_transitions_handle)
                    .expect("Expected `CharacterControlTransitions` to be loaded.");

                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );
            });
    }

    fn transitions() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition::new(
                ControlTransition::Press(ControlTransitionPress::new(
                    ControlAction::Attack,
                    CharacterSequenceId::StandAttack,
                )),
                None,
            ),
            CharacterControlTransition::new(
                ControlTransition::Press(ControlTransitionPress::new(
                    ControlAction::Jump,
                    CharacterSequenceId::Jump,
                )),
                None,
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
            .map(|(entity, _, _, _)| SequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (
            entities,
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
            .map(|(entity, frame_index_clock, _, _)| {
                let frame_index = (*frame_index_clock).value;
                SequenceUpdateEvent::FrameBegin {
                    entity,
                    frame_index,
                }
            })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, CharacterControlTransitionsHandle>,
        WriteStorage<'s, CharacterControlTransitionsSequenceHandle>,
    );
}
