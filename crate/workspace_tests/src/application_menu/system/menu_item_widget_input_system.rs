#[cfg(test)]
mod tests {
    use std::{any, fmt::Debug};

    use amethyst::{
        ecs::{Builder, Entity, World, WorldExt},
        shred::SystemData,
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlBindings,
        ControlInputEvent,
    };
    use strum::IntoEnumIterator;
    use strum_macros::{Display, EnumIter, EnumString};
    use ui_model_spi::play::{Siblings, WidgetStatus};

    use application_menu::{
        MenuEvent, MenuItem, MenuItemWidgetInputSystem, MenuItemWidgetInputSystemData,
    };

    #[test]
    fn does_not_send_event_when_no_input() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::First,
                control_input_event_fn: None,
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Active, WidgetStatus::Idle, WidgetStatus::Idle],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn stays_on_first_item_when_input_up_and_first_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::First,
                control_input_event_fn: Some(press_up),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Active, WidgetStatus::Idle, WidgetStatus::Idle],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn selects_second_item_when_input_down_and_first_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::First,
                control_input_event_fn: Some(press_down),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Active, WidgetStatus::Idle],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn selects_first_item_when_input_up_and_second_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Second,
                control_input_event_fn: Some(press_up),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Active, WidgetStatus::Idle, WidgetStatus::Idle],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn selects_third_item_when_input_down_and_second_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Second,
                control_input_event_fn: Some(press_down),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Idle, WidgetStatus::Active],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn selects_second_item_when_input_up_and_third_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Third,
                control_input_event_fn: Some(press_up),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Active, WidgetStatus::Idle],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn stays_on_third_item_when_input_down_and_third_item_selected() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Third,
                control_input_event_fn: Some(press_down),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Idle, WidgetStatus::Active],
                menu_events: vec![],
            },
        )
    }

    #[test]
    fn sends_select_event_when_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Third,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Idle, WidgetStatus::Active],
                menu_events: vec![MenuEvent::Select(TestIndex::Third)],
            },
        )
    }

    #[test]
    fn sends_close_event_when_input_jump() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::Second,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_statuss: vec![WidgetStatus::Idle, WidgetStatus::Active, WidgetStatus::Idle],
                menu_events: vec![MenuEvent::Close],
            },
        )
    }

    fn run_test(
        SetupParams {
            active_menu_item: setup_active_menu_item,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            widget_statuss: expected_widget_states,
            menu_events,
        }: ExpectedParams<TestIndex>,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                MenuItemWidgetInputSystem::<TestIndex>::new(),
                any::type_name::<MenuItemWidgetInputSystem<TestIndex>>(),
                &[],
            ) // kcov-ignore
            .with_effect(move |world| {
                MenuItemWidgetInputSystemData::<TestIndex>::setup(world);

                // Setup event reader.
                let event_channel_reader = world
                    .write_resource::<EventChannel<MenuEvent<TestIndex>>>()
                    .register_reader(); // kcov-ignore
                world.insert(event_channel_reader);

                let entities = TestIndex::iter()
                    .map(|index| {
                        let widget_status = if index == setup_active_menu_item {
                            WidgetStatus::Active
                        } else {
                            WidgetStatus::Idle
                        };
                        menu_item_entity(world, index, widget_status)
                    })
                    .collect::<Vec<Entity>>();
                {
                    let mut siblingses = world.write_storage::<Siblings>();
                    siblingses
                        .insert(entities[0], Siblings::new(None, Some(entities[1])))
                        .expect("Failed to insert `Siblings` component.");
                    siblingses
                        .insert(
                            entities[1],
                            Siblings::new(Some(entities[0]), Some(entities[2])),
                        )
                        .expect("Failed to insert `Siblings` component.");
                    siblingses
                        .insert(entities[2], Siblings::new(Some(entities[1]), None))
                        .expect("Failed to insert `Siblings` component.");
                }

                world.insert(entities);
            })
            .with_effect(move |world| {
                if let Some(control_input_event_fn) = control_input_event_fn {
                    let entity = world.create_entity().build();
                    world
                        .write_resource::<EventChannel<ControlInputEvent>>()
                        .single_write(control_input_event_fn(entity));
                }
            })
            .with_assertion(move |world| {
                assert_widget_statuses(world, expected_widget_states.clone())
            })
            .with_assertion(move |world| {
                assert_events(world, menu_events.clone());
            })
            .run()
    }

    fn press_up(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::Z,
            value: -1.,
        })
    }

    fn press_down(entity: Entity) -> ControlInputEvent {
        ControlInputEvent::AxisMoved(AxisMoveEventData {
            entity,
            axis: Axis::Z,
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

    fn menu_item_entity(
        world: &mut World,
        menu_item_index: TestIndex,
        widget_status: WidgetStatus,
    ) -> Entity {
        world
            .create_entity()
            .with(MenuItem::new(menu_item_index))
            .with(widget_status)
            .build()
    }

    fn assert_widget_statuses(world: &mut World, expected: Vec<WidgetStatus>) {
        let entities = world.read_resource::<Vec<Entity>>();

        let widget_statuses = world.read_storage::<WidgetStatus>();
        let states = entities
            .iter()
            .map(|entity| {
                *widget_statuses
                    .get(*entity)
                    .expect("Expected entity to have `WidgetStatus` component.")
            })
            .collect::<Vec<WidgetStatus>>();

        assert_eq!(expected, states);
    }

    fn assert_events<I>(world: &mut World, events: Vec<MenuEvent<I>>)
    where
        I: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
    {
        let mut event_channel_reader = &mut world.write_resource::<ReaderId<MenuEvent<I>>>();

        let game_mode_selection_event_channel = world.read_resource::<EventChannel<MenuEvent<I>>>();
        let game_mode_selection_event_iter =
            game_mode_selection_event_channel.read(&mut event_channel_reader);

        let expected_events_iter = events.into_iter();
        expected_events_iter
            .zip(game_mode_selection_event_iter)
            .for_each(|(expected_event, actual)| assert_eq!(expected_event, *actual));
    }

    struct SetupParams {
        active_menu_item: TestIndex,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams<I>
    where
        I: Clone + Copy + Debug + PartialEq + Send + Sync + 'static,
    {
        widget_statuss: Vec<WidgetStatus>,
        menu_events: Vec<MenuEvent<I>>,
    }

    #[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq, Eq)]
    #[strum(serialize_all = "snake_case")]
    enum TestIndex {
        First,
        Second,
        Third,
    }
}
