use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    ecs::{
        Entities, Entity, Join, Read, ReadStorage, Resources, System, SystemData, World, Write,
        WriteStorage,
    },
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use log::debug;
use typename::TypeName as TypeNameTrait;
use typename_derive::TypeName;

use crate::{MenuEvent, MenuItem, MenuItemWidgetState, Siblings};

/// System that processes controller input and generates `MenuEvent<I>`s.
#[derive(Debug, Default, TypeName, new)]
pub struct MenuItemWidgetInputSystem<I>
where
    I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
{
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
    /// PhantomData.
    phantom_data: PhantomData<I>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MenuItemWidgetInputResources<'s, I>
where
    I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `MenuItem` components.
    #[derivative(Debug = "ignore")]
    pub menu_items: ReadStorage<'s, MenuItem<I>>,
    /// `MenuItemWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub menu_item_widget_states: WriteStorage<'s, MenuItemWidgetState>,
    /// `Siblings` components.
    #[derivative(Debug = "ignore")]
    pub siblingses: ReadStorage<'s, Siblings>,
    /// `MenuEvent<I>` channel.
    #[derivative(Debug = "ignore")]
    pub menu_ec: Write<'s, EventChannel<MenuEvent<I>>>,
}

type MenuItemWidgetInputSystemData<'s, I> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    MenuItemWidgetInputResources<'s, I>,
);

impl<I> MenuItemWidgetInputSystem<I>
where
    I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
{
    fn select_previous_menu_item(
        menu_item_widget_states: &mut WriteStorage<'_, MenuItemWidgetState>,
        menu_item_entity: Entity,
        siblings: &Siblings,
    ) {
        if let Some(previous_menu_item) = siblings.previous.as_ref() {
            {
                let menu_item_widget_state = menu_item_widget_states
                    .get_mut(menu_item_entity)
                    .expect("Expected `MenuItemWidgetState` component to exist.");
                *menu_item_widget_state = MenuItemWidgetState::Idle;
            }
            {
                let menu_item_widget_state = menu_item_widget_states
                    .get_mut(*previous_menu_item)
                    .expect("Expected `MenuItemWidgetState` component to exist.");
                *menu_item_widget_state = MenuItemWidgetState::Active;
            }
        }
    }

    fn select_next_menu_item(
        menu_item_widget_states: &mut WriteStorage<'_, MenuItemWidgetState>,
        menu_item_entity: Entity,
        siblings: &Siblings,
    ) {
        if let Some(next_menu_item) = siblings.next.as_ref() {
            {
                let menu_item_widget_state = menu_item_widget_states
                    .get_mut(menu_item_entity)
                    .expect("Expected `MenuItemWidgetState` component to exist.");
                *menu_item_widget_state = MenuItemWidgetState::Idle;
            }
            {
                let menu_item_widget_state = menu_item_widget_states
                    .get_mut(*next_menu_item)
                    .expect("Expected `MenuItemWidgetState` component to exist.");
                *menu_item_widget_state = MenuItemWidgetState::Active;
            }
        }
    }

    fn handle_event(
        MenuItemWidgetInputResources {
            ref entities,
            ref menu_items,
            ref mut menu_item_widget_states,
            ref siblingses,
            ref mut menu_ec,
        }: &mut MenuItemWidgetInputResources<I>,
        event: ControlInputEvent,
    ) {
        // Need to get from `menu_item_widget_states` separately, so that we do not hold an
        // immutable reference. This will then allow us to pass it to lower level functions.
        if let Some((menu_item_entity, siblings)) = (entities, siblingses)
            .join()
            .filter_map(|(entity, siblings)| {
                if let Some(menu_item_widget_state) = menu_item_widget_states.get(entity) {
                    if *menu_item_widget_state == MenuItemWidgetState::Active {
                        return Some((entity, siblings));
                    }
                }
                None
            })
            .next()
        {
            match event {
                ControlInputEvent::AxisMoved(axis_move_event_data) => Self::handle_axis_event(
                    menu_item_widget_states,
                    menu_item_entity,
                    siblings,
                    axis_move_event_data,
                ),
                ControlInputEvent::ControlActionPress(control_action_event_data) => {
                    Self::handle_control_action_event(
                        menu_items,
                        menu_ec,
                        menu_item_entity,
                        control_action_event_data,
                    )
                }
                ControlInputEvent::ControlActionRelease(..) => {}
            }
        }
    }

    fn handle_axis_event(
        menu_item_widget_states: &mut WriteStorage<'_, MenuItemWidgetState>,
        menu_item_entity: Entity,
        siblings: &Siblings,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let menu_item_widget_state = *menu_item_widget_states
            .get(menu_item_entity)
            .expect("Expected `MenuItemWidgetState` component to exist.");
        match (menu_item_widget_state, axis_move_event_data.axis) {
            (MenuItemWidgetState::Active, Axis::Z) if axis_move_event_data.value < 0. => {
                Self::select_previous_menu_item(
                    menu_item_widget_states,
                    menu_item_entity,
                    siblings,
                );
            }
            (MenuItemWidgetState::Active, Axis::Z) if axis_move_event_data.value > 0. => {
                Self::select_next_menu_item(menu_item_widget_states, menu_item_entity, siblings);
            }
            _ => {}
        }
    }

    fn handle_control_action_event(
        menu_items: &ReadStorage<'_, MenuItem<I>>,
        menu_ec: &mut EventChannel<MenuEvent<I>>,
        menu_item_entity: Entity,
        control_action_event_data: ControlActionEventData,
    ) {
        let game_mode_selection_event = match control_action_event_data.control_action {
            ControlAction::Jump => Some(MenuEvent::Close),
            ControlAction::Attack => {
                let menu_item = menu_items
                    .get(menu_item_entity)
                    .expect("Expected `MenuItem` component to exist.");

                Some(MenuEvent::Select(menu_item.index))
            }
            _ => None,
        };

        if let Some(game_mode_selection_event) = game_mode_selection_event {
            debug!(
                "Sending game_mode selection event: {:?}",
                &game_mode_selection_event // kcov-ignore
            );
            menu_ec.single_write(game_mode_selection_event);
        }
    }
}

impl<'s, I> System<'s> for MenuItemWidgetInputSystem<I>
where
    I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
{
    type SystemData = MenuItemWidgetInputSystemData<'s, I>;

    fn run(&mut self, (control_input_ec, mut sibling_input_resources): Self::SystemData) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                Self::handle_event(&mut sibling_input_resources, *ev);
            });
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.control_input_event_rid = Some(
            res.fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use amethyst::{
        ecs::{Builder, Entity, SystemData, World},
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
    use typename::TypeName as TypeNameTrait;
    use typename_derive::TypeName;

    use super::{MenuItemWidgetInputSystem, MenuItemWidgetInputSystemData};
    use crate::{MenuEvent, MenuItem, MenuItemWidgetState, Siblings};

    #[test]
    fn does_not_send_event_when_no_input() -> Result<(), Error> {
        run_test(
            SetupParams {
                active_menu_item: TestIndex::First,
                control_input_event_fn: None,
            },
            ExpectedParams {
                widget_states: vec![
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                ],
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
                widget_states: vec![
                    MenuItemWidgetState::Idle,
                    MenuItemWidgetState::Active,
                    MenuItemWidgetState::Idle,
                ],
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
            widget_states: expected_widget_states,
            menu_events,
        }: ExpectedParams<TestIndex>,
    ) -> Result<(), Error> {
        AmethystApplication::ui_base::<ControlBindings>()
            .with_system(
                MenuItemWidgetInputSystem::<TestIndex>::new(),
                MenuItemWidgetInputSystem::<TestIndex>::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(move |world| {
                MenuItemWidgetInputSystemData::<TestIndex>::setup(&mut world.res);

                // Setup event reader.
                let event_channel_reader = world
                    .write_resource::<EventChannel<MenuEvent<TestIndex>>>()
                    .register_reader(); // kcov-ignore
                world.insert(event_channel_reader);

                let entities = TestIndex::iter()
                    .map(|index| {
                        let widget_state = if index == setup_active_menu_item {
                            MenuItemWidgetState::Active
                        } else {
                            MenuItemWidgetState::Idle
                        };
                        menu_item_entity(world, index, widget_state)
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
                assert_menu_item_widget_states(world, expected_widget_states.clone())
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
        widget_state: MenuItemWidgetState,
    ) -> Entity {
        world
            .create_entity()
            .with(MenuItem::new(menu_item_index))
            .with(widget_state)
            .build()
    }

    fn assert_menu_item_widget_states(world: &mut World, expected: Vec<MenuItemWidgetState>) {
        let entities = world.read_resource::<Vec<Entity>>();

        let menu_item_widget_states = world.read_storage::<MenuItemWidgetState>();
        let states = entities
            .iter()
            .map(|entity| {
                *menu_item_widget_states
                    .get(*entity)
                    .expect("Expected entity to have `MenuItemWidgetState` component.")
            })
            .collect::<Vec<MenuItemWidgetState>>();

        assert_eq!(expected, states);
    }

    fn assert_events<I>(world: &mut World, events: Vec<MenuEvent<I>>)
    where
        I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
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
        I: Clone + Copy + Debug + PartialEq + Send + Sync + TypeNameTrait + 'static,
    {
        widget_states: Vec<MenuItemWidgetState>,
        menu_events: Vec<MenuEvent<I>>,
    }

    #[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, PartialEq, Eq, TypeName)]
    #[strum(serialize_all = "snake_case")]
    enum TestIndex {
        First,
        Second,
        Third,
    }
}
