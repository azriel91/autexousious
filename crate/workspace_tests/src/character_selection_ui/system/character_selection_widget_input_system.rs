#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::{AssetQueries, AutexousiousApplication};
    use asset_model::config::AssetType;
    use assets_test::CHAR_BAT_SLUG;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use game_input::InputControlled;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use object_type::ObjectType;
    use typename::TypeName;

    use character_selection_ui::{
        CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetInputSystemData,
        CharacterSelectionWidgetState,
    };

    #[test]
    fn does_not_send_event_when_controller_input_empty() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Inactive,
                character_selection_fn: char_random,
                control_input_event_fn: None,
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Inactive,
                character_selection_fn: char_random,
                character_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn updates_widget_inactive_to_character_select_when_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Inactive,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_random,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Join { controller_id: 123 }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_character_select_to_ready_and_sends_event_when_input_attack(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Ready,
                character_selection_fn: char_bat,
                character_selection_events_fn: |world| {
                    let bat_asset_id = AssetQueries::id(world, &*CHAR_BAT_SLUG);
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(bat_asset_id),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_last_character_when_input_left_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_left),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: |world| {
                    let last_char =
                        AssetQueries::last_id(world, AssetType::Object(ObjectType::Character));
                    CharacterSelection::Id(last_char)
                },
                character_selection_events_fn: |world| {
                    let last_char =
                        AssetQueries::last_id(world, AssetType::Object(ObjectType::Character));
                    vec![CharacterSelectionEvent::Switch {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(last_char),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_first_character_when_input_right_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: |world| {
                    let first_char =
                        AssetQueries::first_id(world, AssetType::Object(ObjectType::Character));
                    CharacterSelection::Id(first_char)
                },
                character_selection_events_fn: |world| {
                    let first_char =
                        AssetQueries::first_id(world, AssetType::Object(ObjectType::Character));
                    vec![CharacterSelectionEvent::Switch {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(first_char),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_random_when_input_right_and_selection_last_character() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_random,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Switch {
                        controller_id: 123,
                        character_selection: CharacterSelection::Random,
                    }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_ready_to_character_select_and_sends_event_when_input_jump(
    ) -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Ready,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Deselect { controller_id: 123 }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_character_select_to_inactive_when_input_jump() -> Result<(), Error> {
        run_test(
            SetupParams {
                character_selection_widget_state: CharacterSelectionWidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                character_selection_widget_state: CharacterSelectionWidgetState::Inactive,
                character_selection_fn: char_bat,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Leave { controller_id: 123 }]
                },
            },
        )
    }

    fn run_test(
        SetupParams {
            character_selection_widget_state: setup_character_selection_widget_state,
            character_selection_fn: setup_character_selection_fn,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            character_selection_widget_state: expected_character_selection_widget_state,
            character_selection_fn: expected_character_selection_fn,
            character_selection_events_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                CharacterSelectionWidgetInputSystem::new(),
                CharacterSelectionWidgetInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                CharacterSelectionWidgetInputSystemData::setup(world);

                let setup_character_selection = setup_character_selection_fn(world);
                let entity = widget_entity(
                    world,
                    setup_character_selection,
                    setup_character_selection_widget_state,
                );
                world.insert(entity);

                let event_channel_reader = world
                    .write_resource::<EventChannel<CharacterSelectionEvent>>()
                    .register_reader(); // kcov-ignore

                world.insert(event_channel_reader);
            })
            .with_effect(move |world| {
                if let Some(control_input_event_fn) = control_input_event_fn {
                    let entity = *world.read_resource::<Entity>();
                    world
                        .write_resource::<EventChannel<ControlInputEvent>>()
                        .single_write(control_input_event_fn(entity));
                }
            })
            .with_assertion(move |world| {
                let expected_character_selection = expected_character_selection_fn(world);
                assert_widget(
                    world,
                    expected_character_selection,
                    expected_character_selection_widget_state,
                )
            })
            .with_assertion(move |world| {
                let character_selection_events = character_selection_events_fn(world);
                assert_events(world, character_selection_events);
            })
            .run_isolated()
    }

    fn press_left(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::X,
            value: -1.,
        })
    }

    fn press_right(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::X,
            value: 1.,
        })
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

    fn char_random(_world: &mut World) -> CharacterSelection {
        CharacterSelection::Random
    }

    fn char_bat(world: &mut World) -> CharacterSelection {
        let bat_asset_id = AssetQueries::id(&*world, &*CHAR_BAT_SLUG);
        CharacterSelection::Id(bat_asset_id)
    }

    fn widget_entity(
        world: &mut World,
        character_selection: CharacterSelection,
        character_selection_widget_state: CharacterSelectionWidgetState,
    ) -> Entity {
        world
            .create_entity()
            .with(character_selection)
            .with(character_selection_widget_state)
            .with(InputControlled::new(123))
            .build()
    }

    fn assert_widget(
        world: &mut World,
        expected_character_selection: CharacterSelection,
        expected_character_selection_widget_state: CharacterSelectionWidgetState,
    ) {
        let widget_entity = world.read_resource::<Entity>();

        let character_selections = world.read_storage::<CharacterSelection>();
        let character_selection = character_selections
            .get(*widget_entity)
            .expect("Expected entity to have `CharacterSelection` component.");

        assert_eq!(expected_character_selection, *character_selection);

        let character_selection_widget_states =
            world.read_storage::<CharacterSelectionWidgetState>();
        let character_selection_widget_state = character_selection_widget_states
            .get(*widget_entity)
            .expect("Expected entity to have `CharacterSelectionWidgetState` component.");

        assert_eq!(
            expected_character_selection_widget_state,
            *character_selection_widget_state
        );
    }

    fn assert_events(world: &mut World, events: Vec<CharacterSelectionEvent>) {
        let mut event_channel_reader =
            &mut world.write_resource::<ReaderId<CharacterSelectionEvent>>();

        let character_selection_event_channel =
            world.read_resource::<EventChannel<CharacterSelectionEvent>>();
        let actual_events = character_selection_event_channel
            .read(&mut event_channel_reader)
            .collect::<Vec<&CharacterSelectionEvent>>();

        let expected_events = events.iter().collect::<Vec<&CharacterSelectionEvent>>();
        assert_eq!(expected_events, actual_events);
    }

    struct SetupParams {
        character_selection_widget_state: CharacterSelectionWidgetState,
        character_selection_fn: fn(&mut World) -> CharacterSelection,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams {
        character_selection_widget_state: CharacterSelectionWidgetState,
        character_selection_fn: fn(&mut World) -> CharacterSelection,
        character_selection_events_fn: fn(&mut World) -> Vec<CharacterSelectionEvent>,
    }
}
