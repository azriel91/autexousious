#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
    use typename::TypeName;

    use character_selection_ui::{
        CharacterSelectionInputSystem, CharacterSelectionInputSystemData, CharacterSelectionWidget,
        WidgetState,
    };

    #[test]
    fn does_not_send_event_when_controller_input_empty() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_states: vec![WidgetState::Inactive, WidgetState::Inactive],
                control_input_event_fn: None,
            },
            ExpectedParams {
                character_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn does_not_send_return_event_when_controller_input_jump_and_not_all_inactive(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_states: vec![WidgetState::Ready, WidgetState::Inactive],
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                character_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn send_return_event_when_controller_input_jump_and_all_inactive() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_states: vec![WidgetState::Inactive, WidgetState::Inactive],
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                character_selection_events_fn: |_world| vec![CharacterSelectionEvent::Return],
            },
        )
    }

    #[test]
    fn sends_confirm_event_when_widget_ready_and_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_states: vec![WidgetState::Ready, WidgetState::Ready],
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                character_selection_events_fn: |_world| vec![CharacterSelectionEvent::Confirm],
            },
        )
    }

    #[test]
    fn does_not_send_event_when_not_enough_players() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_states: vec![WidgetState::Ready, WidgetState::Inactive],
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                character_selection_events_fn: |_world| vec![],
            },
        )
    }

    fn run_test(
        SetupParams {
            widget_states: setup_widget_states,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            character_selection_events_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                CharacterSelectionInputSystem::new(),
                CharacterSelectionInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                CharacterSelectionInputSystemData::setup(world);

                let entities = setup_widget_states
                    .iter()
                    .map(|setup_widget_state| {
                        let character_selection = CharacterSelection::Random;
                        widget_entity(world, *setup_widget_state, character_selection)
                    })
                    .collect::<Vec<Entity>>();
                world.insert(entities);

                let event_channel_reader = world
                    .write_resource::<EventChannel<CharacterSelectionEvent>>()
                    .register_reader(); // kcov-ignore

                world.insert(event_channel_reader);
            })
            .with_effect(move |world| {
                if let Some(control_input_event_fn) = control_input_event_fn {
                    let entities = world.read_resource::<Vec<Entity>>();
                    let entity = entities
                        .iter()
                        .next()
                        .expect("Expected at least one character selection widget entity.");
                    world
                        .write_resource::<EventChannel<ControlInputEvent>>()
                        .single_write(control_input_event_fn(*entity));
                }
            })
            .with_assertion(move |world| {
                let character_selection_events = character_selection_events_fn(world);
                assert_events(world, character_selection_events);
            })
            .run()
    }

    fn press_jump(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Jump,
        })
    }

    fn press_attack(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::ControlActionPress(ControlActionEventData {
            entity,
            control_action: ControlAction::Attack,
        })
    }

    fn empty_events(_world: &mut World) -> Vec<CharacterSelectionEvent> {
        vec![]
    }

    fn widget_entity(
        world: &mut World,
        widget_state: WidgetState,
        character_selection: CharacterSelection,
    ) -> Entity {
        world
            .create_entity()
            .with(CharacterSelectionWidget::new(
                widget_state,
                character_selection,
            ))
            .build()
    }

    fn assert_events(world: &mut World, expected_events: Vec<CharacterSelectionEvent>) {
        let mut event_channel_reader =
            &mut world.write_resource::<ReaderId<CharacterSelectionEvent>>();

        let character_selection_event_channel =
            world.read_resource::<EventChannel<CharacterSelectionEvent>>();
        let actual_events = character_selection_event_channel
            .read(&mut event_channel_reader)
            .collect::<Vec<&CharacterSelectionEvent>>();

        let expected_events = expected_events
            .iter()
            .collect::<Vec<&CharacterSelectionEvent>>();

        assert_eq!(expected_events, actual_events)
    }

    struct SetupParams {
        widget_states: Vec<WidgetState>,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams {
        character_selection_events_fn: fn(&mut World) -> Vec<CharacterSelectionEvent>,
    }
}
