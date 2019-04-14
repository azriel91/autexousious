use amethyst::{
    assets::AssetStorage,
    ecs::{Read, ReadStorage, System, SystemData, WriteStorage},
    shred::Resources,
    shrev::{EventChannel, ReaderId},
};
use character_model::loaded::{
    CharacterControlTransitions, CharacterControlTransitionsSequence,
    CharacterControlTransitionsSequenceHandle,
};
use derivative::Derivative;
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::play::{FrameIndexClock, SequenceUpdateEvent};
use shred_derive::SystemData;

/// Updates frame components.
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
    /// `FrameIndexClock` component storage.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: ReadStorage<'s, FrameIndexClock>,
    /// `CharacterControlTransitions` component storage.
    #[derivative(Debug = "ignore")]
    pub character_control_transitionses: WriteStorage<'s, CharacterControlTransitions>,
}

impl<'s> System<'s> for CharacterControlTransitionsUpdateSystem {
    type SystemData = CharacterControlTransitionsUpdateSystemData<'s>;

    fn run(
        &mut self,
        CharacterControlTransitionsUpdateSystemData {
            sequence_update_ec,
            character_cts_handles,
            character_cts_assets,
            frame_index_clocks,
            mut character_control_transitionses,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id.as_mut().expect(
                    "Expected reader ID to exist for CharacterControlTransitionsUpdateSystem.",
                ),
            )
            .for_each(|ev| {
                let (entity, frame_index) = match ev {
                    SequenceUpdateEvent::SequenceBegin { entity }
                    | SequenceUpdateEvent::FrameBegin { entity } => {
                        let frame_index_clock = frame_index_clocks
                            .get(*entity)
                            .expect("Expected entity to have a `FrameIndexClock` component.");
                        let frame_index = (*frame_index_clock).value;

                        (entity, frame_index)
                    }
                    SequenceUpdateEvent::SequenceEnd { entity } => (entity, 0),
                };

                let character_cts_handle = character_cts_handles.get(*entity).expect(
                    "Expected entity to have a `CharacterControlTransitionsSequenceHandle` \
                     component.",
                );
                let character_control_transitions_sequence = character_cts_assets
                    .get(character_cts_handle)
                    .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

                let character_control_transitions =
                    &character_control_transitions_sequence[frame_index];

                // TODO: Store character_control_transitions as an asset.
                character_control_transitionses
                    .insert(*entity, character_control_transitions.clone())
                    .expect("Failed to insert `CharacterControlTransitions` component.");
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
        ecs::{Entities, Join, ReadStorage, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, SequenceQueries};
    use assets_test::ASSETS_CHAR_BAT_SLUG;
    use character_model::{
        config::CharacterSequenceId,
        loaded::{
            CharacterControlTransition, CharacterControlTransitions,
            CharacterControlTransitionsSequenceHandle,
        },
    };
    use game_input_model::ControlAction;
    use sequence_model::{
        loaded::{ControlTransition, ControlTransitionPress},
        play::{FrameIndexClock, SequenceUpdateEvent},
    };

    use super::CharacterControlTransitionsUpdateSystem;

    #[test]
    fn updates_all_frame_components_on_sequence_begin_event() -> Result<(), Error> {
        let test_name = "updates_all_frame_components_on_sequence_begin_event";
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
                    // third frame in the sequence, though it doesn't make sense for sequence begin
                    2,
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
    fn updates_all_frame_components_on_frame_begin_event() -> Result<(), Error> {
        let test_name = "updates_all_frame_components_on_frame_begin_event";
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
            _character_control_transitionses,
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
        let (character_control_transitionses, sequence_statuses) = world.system_data::<(
            WriteStorage<CharacterControlTransitions>,
            ReadStorage<CharacterSequenceId>,
        )>();

        (&character_control_transitionses, &sequence_statuses)
            .join()
            .for_each(|(character_control_transitions, _sequence_status)| {
                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );
            });
    }

    fn transitions() -> CharacterControlTransitions {
        CharacterControlTransitions::new(vec![
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
        ])
    }

    fn send_events(world: &mut World, events: Vec<SequenceUpdateEvent>) {
        let mut ec = world.write_resource::<EventChannel<SequenceUpdateEvent>>();
        ec.iter_write(events.into_iter())
    }

    fn sequence_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (entities, frame_index_clocks, character_control_transitionses, character_cts_handles) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &character_control_transitionses,
            &character_cts_handles,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::SequenceBegin { entity })
            .collect::<Vec<_>>()
    }

    fn frame_begin_events(world: &mut World) -> Vec<SequenceUpdateEvent> {
        let (entities, frame_index_clocks, character_control_transitionses, character_cts_handles) =
            world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &character_control_transitionses,
            &character_cts_handles,
        )
            .join()
            .map(|(entity, _, _, _)| SequenceUpdateEvent::FrameBegin { entity })
            .collect::<Vec<_>>()
    }

    type TestSystemData<'s> = (
        Entities<'s>,
        WriteStorage<'s, FrameIndexClock>,
        WriteStorage<'s, CharacterControlTransitions>,
        WriteStorage<'s, CharacterControlTransitionsSequenceHandle>,
    );
}
