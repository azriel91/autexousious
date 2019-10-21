#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::{
        config::AssetType,
        loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::MAP_FADE_SLUG;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use map_selection_model::{MapSelection, MapSelectionEvent};
    use typename::TypeName;

    use map_selection_ui::{
        MapSelectionWidget, MapSelectionWidgetInputSystem, MapSelectionWidgetInputSystemData,
        WidgetState,
    };

    #[test]
    fn does_not_send_event_when_no_input() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: None,
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                map_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn selects_last_map_when_input_left_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: Some(press_left),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let last_map = last_map(world);
                    MapSelection::Id(last_map)
                },
                map_selection_events_fn: |world| {
                    let last_map = last_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Id(last_map),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_first_map_when_input_right_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let first_map = first_map(world);
                    MapSelection::Id(first_map)
                },
                map_selection_events_fn: |world| {
                    let first_map = first_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Id(first_map),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_random_when_input_right_and_selection_last_map() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: |world| {
                    let last_map = last_map(world);
                    MapSelection::Id(last_map)
                },
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_random,
                map_selection_events_fn: |world| {
                    let first_map = first_map(world);
                    vec![MapSelectionEvent::Switch {
                        map_selection: MapSelection::Random(Some(first_map)),
                    }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_map_select_to_ready_and_sends_event_when_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |world| {
                    vec![MapSelectionEvent::Select {
                        map_selection: map_selection_fade(world),
                    }]
                },
            },
        )
    }

    #[test]
    fn updates_widget_ready_to_map_select_and_sends_event_when_input_jump() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Deselect],
            },
        )
    }

    #[test]
    fn sends_confirm_event_when_widget_ready_and_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::Ready,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Confirm],
            },
        )
    }

    #[test]
    fn send_return_event_when_controller_input_jump_and_widget_map_select() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::MapSelect,
                map_selection_fn: map_selection_fade,
                map_selection_events_fn: |_world| vec![MapSelectionEvent::Return],
            },
        )
    }

    fn run_test(
        SetupParams {
            widget_state: widget_entity_state,
            map_selection_fn: setup_map_selection_fn,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            widget_state: expected_widget_state,
            map_selection_fn: expected_map_selection_fn,
            map_selection_events_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                MapSelectionWidgetInputSystem::new(),
                MapSelectionWidgetInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                MapSelectionWidgetInputSystemData::setup(world);

                // Setup event reader.
                let event_channel_reader = world
                    .write_resource::<EventChannel<MapSelectionEvent>>()
                    .register_reader(); // kcov-ignore
                world.insert(event_channel_reader);

                let map_selection = setup_map_selection_fn(world);
                let entity = widget_entity(world, widget_entity_state, map_selection);
                world.insert(entity);
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
                let expected_map_selection = expected_map_selection_fn(world);
                assert_widget(
                    world,
                    MapSelectionWidget::new(expected_widget_state, expected_map_selection),
                )
            })
            .with_assertion(move |world| {
                let map_selection_events = map_selection_events_fn(world);
                assert_events(world, map_selection_events);
            })
            .run()
    }

    fn map_selection_fade(world: &mut World) -> MapSelection {
        let fade_map = fade_map(world);
        MapSelection::Id(fade_map)
    }

    fn map_selection_random(world: &mut World) -> MapSelection {
        let first_map = first_map(world);
        MapSelection::Random(Some(first_map))
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

    fn empty_events(_world: &mut World) -> Vec<MapSelectionEvent> {
        vec![]
    }

    fn first_map(world: &mut World) -> AssetId {
        world
            .read_resource::<AssetTypeMappings>()
            .iter_ids(&AssetType::Map)
            .copied()
            .next()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn last_map(world: &mut World) -> AssetId {
        world
            .read_resource::<AssetTypeMappings>()
            .iter_ids(&AssetType::Map)
            .copied()
            .next_back()
            .expect("Expected at least one map to be loaded.")
            .into()
    }

    fn fade_map(world: &mut World) -> AssetId {
        world
            .read_resource::<AssetIdMappings>()
            .id(&*MAP_FADE_SLUG)
            .copied()
            .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", &*MAP_FADE_SLUG))
    }

    fn widget_entity(
        world: &mut World,
        widget_state: WidgetState,
        map_selection: MapSelection,
    ) -> Entity {
        world
            .create_entity()
            .with(MapSelectionWidget::new(widget_state, map_selection))
            .build()
    }

    fn assert_widget(world: &mut World, expected: MapSelectionWidget) {
        let widget_entity = world.read_resource::<Entity>();

        let widgets = world.read_storage::<MapSelectionWidget>();
        let widget = widgets
            .get(*widget_entity)
            .expect("Expected entity to have `MapSelectionWidget` component.");

        assert_eq!(expected, *widget);
    }

    fn assert_events(world: &mut World, events: Vec<MapSelectionEvent>) {
        let mut event_channel_reader = &mut world.write_resource::<ReaderId<MapSelectionEvent>>();

        let map_selection_event_channel = world.read_resource::<EventChannel<MapSelectionEvent>>();
        let actual_events = map_selection_event_channel
            .read(&mut event_channel_reader)
            .collect::<Vec<&MapSelectionEvent>>();

        let expected_events = events.iter().collect::<Vec<&MapSelectionEvent>>();
        assert_eq!(expected_events, actual_events);
    }

    struct SetupParams {
        widget_state: WidgetState,
        map_selection_fn: fn(&mut World) -> MapSelection,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams {
        widget_state: WidgetState,
        map_selection_fn: fn(&mut World) -> MapSelection,
        map_selection_events_fn: fn(&mut World) -> Vec<MapSelectionEvent>,
    }
}
