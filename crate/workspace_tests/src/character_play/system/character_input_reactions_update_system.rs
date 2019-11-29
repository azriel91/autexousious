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
        CharacterControlTransition, CharacterInputReactions, CharacterInputReactionsHandle,
        CharacterIrsHandle,
    };
    use game_input_model::ControlAction;
    use sequence_model::{
        loaded::{ActionPress, ControlTransition, InputReactions, SequenceId},
        play::{FrameIndexClock, SequenceUpdateEvent},
    };

    use character_play::CharacterInputReactionsUpdateSystem;

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
            .with_system(CharacterInputReactionsUpdateSystem::new(), "", &[])
            .with_effect(move |world| {
                let character_irs_handle = SequenceQueries::character_irs_handle(
                    world,
                    &*CHAR_BAT_SLUG,
                    SequenceId::new(1),
                );
                initial_values(world, frame_index_clock, character_irs_handle)
            })
            .with_effect(move |world| {
                let events = sequence_update_events_fn(world);
                send_events(world, events);
            })
            .with_assertion(|world| expect_transitions(world, transitions()))
            .run_isolated()
    }

    fn initial_values(
        world: &mut World,
        frame_index_clock_setup: FrameIndexClock,
        character_irs_handle_initial: CharacterIrsHandle,
    ) {
        let (
            _entities,
            _sequence_ids,
            mut frame_index_clocks,
            _character_input_reactions_handles,
            mut character_irs_handles,
        ) = world.system_data::<TestSystemData>();

        (&mut frame_index_clocks, &mut character_irs_handles)
            .join()
            // kcov-ignore-start
            .for_each(|(frame_index_clock, character_irs_handle)| {
                *frame_index_clock = frame_index_clock_setup;

                *character_irs_handle = character_irs_handle_initial.clone();
            });
        // kcov-ignore-end
    }

    fn expect_transitions(
        world: &mut World,
        expected_character_input_reactions: CharacterInputReactions,
    ) {
        let (
            character_input_reactions_assets,
            character_input_reactions_handles,
            sequence_statuses,
        ) = world.system_data::<(
            Read<AssetStorage<CharacterInputReactions>>,
            ReadStorage<CharacterInputReactionsHandle>,
            ReadStorage<SequenceId>,
        )>();

        (&character_input_reactions_handles, &sequence_statuses)
            .join()
            // kcov-ignore-start
            .for_each(|(character_input_reactions_handle, _sequence_status)| {
                let character_input_reactions = character_input_reactions_assets
                    .get(character_input_reactions_handle)
                    .expect("Expected `CharacterInputReactions` to be loaded.");

                assert_eq!(
                    &expected_character_input_reactions,
                    character_input_reactions
                );
            });
        // kcov-ignore-end
    }

    fn transitions() -> CharacterInputReactions {
        CharacterInputReactions::new(InputReactions::new(vec![
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
            character_input_reactions_handles,
            character_irs_handles,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &sequence_ids,
            &frame_index_clocks,
            &character_input_reactions_handles,
            &character_irs_handles,
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
            character_input_reactions_handles,
            character_irs_handles,
        ) = world.system_data::<TestSystemData>();

        (
            &entities,
            &frame_index_clocks,
            &character_input_reactions_handles,
            &character_irs_handles,
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
        WriteStorage<'s, CharacterInputReactionsHandle>,
        WriteStorage<'s, CharacterIrsHandle>,
    );
}
