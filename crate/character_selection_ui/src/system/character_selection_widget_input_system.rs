use amethyst::{
    ecs::{Read, ReadStorage, SystemData, World, Write, WriteStorage},
    shred::{ResourceId, Resources, System, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use game_model::loaded::CharacterPrefabs;
use log::debug;

use typename_derive::TypeName;

use crate::{CharacterSelectionWidget, WidgetState};

/// System that processes controller input and generates `CharacterSelectionEvent`s.
///
/// This is not private because consumers may use `CharacterSelectionWidgetInputSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionWidgetInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub(crate) struct CharacterSelectionWidgetInputResources<'s> {
    /// `CharacterSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_widgets: WriteStorage<'s, CharacterSelectionWidget>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `Character` assets.
    #[derivative(Debug = "ignore")]
    pub character_prefabs: Read<'s, CharacterPrefabs>,
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Write<'s, EventChannel<CharacterSelectionEvent>>,
}

type CharacterSelectionWidgetInputSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    CharacterSelectionWidgetInputResources<'s>,
);

impl CharacterSelectionWidgetInputSystem {
    fn select_previous_character(
        character_prefabs: &CharacterPrefabs,
        widget: &mut CharacterSelectionWidget,
    ) -> CharacterSelection {
        let first_character_slug = character_prefabs
            .keys()
            .next()
            .expect("Expected at least one character to be loaded.");
        let last_character_slug = character_prefabs
            .keys()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(ref character_slug) => {
                if character_slug == first_character_slug {
                    CharacterSelection::Random
                } else {
                    let next_character = character_prefabs
                        .keys()
                        .rev()
                        .skip_while(|slug| slug != &character_slug)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character.clone())
                    } else {
                        CharacterSelection::Random
                    }
                }
            }
            CharacterSelection::Random => CharacterSelection::Id(last_character_slug.clone()),
        };
        widget.selection.clone()
    }

    fn select_next_character(
        character_prefabs: &CharacterPrefabs,
        widget: &mut CharacterSelectionWidget,
    ) -> CharacterSelection {
        let first_character_slug = character_prefabs
            .keys()
            .next()
            .expect("Expected at least one character to be loaded.");
        let last_character_slug = character_prefabs
            .keys()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(ref character_slug) => {
                if character_slug == last_character_slug {
                    CharacterSelection::Random
                } else {
                    let next_character = character_prefabs
                        .keys()
                        .skip_while(|slug| slug != &character_slug)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character.clone())
                    } else {
                        CharacterSelection::Random
                    }
                }
            }
            CharacterSelection::Random => CharacterSelection::Id(first_character_slug.clone()),
        };
        widget.selection.clone()
    }

    fn handle_event(
        CharacterSelectionWidgetInputResources {
            ref mut character_selection_widgets,
            ref input_controlleds,
            ref character_prefabs,
            ref mut character_selection_ec,
        }: &mut CharacterSelectionWidgetInputResources,
        event: ControlInputEvent,
    ) {
        match event {
            ControlInputEvent::AxisMoved(axis_move_event_data) => {
                if let (Some(character_selection_widget), Some(input_controlled)) = (
                    character_selection_widgets.get_mut(axis_move_event_data.entity),
                    input_controlleds.get(axis_move_event_data.entity),
                ) {
                    Self::handle_axis_event(
                        &character_prefabs,
                        character_selection_ec,
                        character_selection_widget,
                        *input_controlled,
                        axis_move_event_data,
                    )
                }
            }
            ControlInputEvent::ControlActionPress(control_action_event_data) => {
                if let (Some(character_selection_widget), Some(input_controlled)) = (
                    character_selection_widgets.get_mut(control_action_event_data.entity),
                    input_controlleds.get(control_action_event_data.entity),
                ) {
                    Self::handle_control_action_event(
                        character_selection_ec,
                        character_selection_widget,
                        *input_controlled,
                        control_action_event_data,
                    )
                }
            }
            ControlInputEvent::ControlActionRelease(..) => {}
        }
    }

    fn handle_axis_event(
        character_prefabs: &CharacterPrefabs,
        character_selection_ec: &mut EventChannel<CharacterSelectionEvent>,
        character_selection_widget: &mut CharacterSelectionWidget,
        input_controlled: InputControlled,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let character_selection =
            match (character_selection_widget.state, axis_move_event_data.axis) {
                (WidgetState::CharacterSelect, Axis::X) if axis_move_event_data.value < 0. => Some(
                    Self::select_previous_character(character_prefabs, character_selection_widget),
                ),
                (WidgetState::CharacterSelect, Axis::X) if axis_move_event_data.value > 0. => Some(
                    Self::select_next_character(character_prefabs, character_selection_widget),
                ),
                _ => None,
            };

        if let Some(character_selection) = character_selection {
            let character_selection_event = CharacterSelectionEvent::Switch {
                controller_id: input_controlled.controller_id,
                character_selection,
            };

            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event // kcov-ignore
            );
            character_selection_ec.single_write(character_selection_event)
        }
    }

    fn handle_control_action_event(
        character_selection_ec: &mut EventChannel<CharacterSelectionEvent>,
        character_selection_widget: &mut CharacterSelectionWidget,
        input_controlled: InputControlled,
        control_action_event_data: ControlActionEventData,
    ) {
        let character_selection_event = match (
            character_selection_widget.state,
            control_action_event_data.control_action,
        ) {
            (WidgetState::Inactive, ControlAction::Attack) => {
                debug!("Controller {} active.", input_controlled.controller_id);
                character_selection_widget.state = WidgetState::CharacterSelect;

                Some(CharacterSelectionEvent::Join {
                    controller_id: input_controlled.controller_id,
                })
            }
            (WidgetState::CharacterSelect, ControlAction::Jump) => {
                debug!("Controller {} inactive.", input_controlled.controller_id);
                character_selection_widget.state = WidgetState::Inactive;

                Some(CharacterSelectionEvent::Leave {
                    controller_id: input_controlled.controller_id,
                })
            }
            (WidgetState::CharacterSelect, ControlAction::Attack) => {
                debug!("Controller {} ready.", input_controlled.controller_id);
                character_selection_widget.state = WidgetState::Ready;

                Some(CharacterSelectionEvent::Select {
                    controller_id: input_controlled.controller_id,
                    character_selection: character_selection_widget.selection.clone(),
                })
            }
            (WidgetState::Ready, ControlAction::Jump) => {
                character_selection_widget.state = WidgetState::CharacterSelect;

                Some(CharacterSelectionEvent::Deselect {
                    controller_id: input_controlled.controller_id,
                })
            }
            _ => None,
        };

        if let Some(character_selection_event) = character_selection_event {
            debug!(
                "Sending character selection event: {:?}",
                &character_selection_event // kcov-ignore
            );
            character_selection_ec.single_write(character_selection_event)
        }
    }
}

impl<'s> System<'s> for CharacterSelectionWidgetInputSystem {
    type SystemData = CharacterSelectionWidgetInputSystemData<'s>;

    fn run(
        &mut self,
        (control_input_ec, mut character_selection_widget_input_resources): Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                Self::handle_event(&mut character_selection_widget_input_resources, *ev);
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
    use amethyst::{
        ecs::{Builder, Entity, SystemData, World},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use asset_model::config::AssetSlug;
    use assets_test::CHAR_BAT_SLUG;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use game_input::InputControlled;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use game_model::loaded::CharacterPrefabs;
    use typename::TypeName;

    use super::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetInputSystemData};
    use crate::{CharacterSelectionWidget, WidgetState};

    #[test]
    fn does_not_send_event_when_controller_input_empty() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Inactive,
                character_selection_fn: char_random,
                control_input_event_fn: None,
            },
            ExpectedParams {
                widget_state: WidgetState::Inactive,
                character_selection_fn: char_random,
                character_selection_events_fn: empty_events,
            },
        )
    }

    #[test]
    fn updates_widget_inactive_to_character_select_when_input_attack() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::Inactive,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::CharacterSelect,
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
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_attack),
            },
            ExpectedParams {
                widget_state: WidgetState::Ready,
                character_selection_fn: char_bat,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Select {
                        controller_id: 123,
                        character_selection: CharacterSelection::Id(CHAR_BAT_SLUG.clone()),
                    }]
                },
            },
        )
    }

    #[test]
    fn selects_last_character_when_input_left_and_selection_random() -> Result<(), Error> {
        run_test(
            SetupParams {
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_left),
            },
            ExpectedParams {
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: |world| {
                    let last_char = last_character(world);
                    CharacterSelection::Id(last_char)
                },
                character_selection_events_fn: |world| {
                    let last_char = last_character(world);
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
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: char_random,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: |world| {
                    let first_char = first_character(world);
                    CharacterSelection::Id(first_char)
                },
                character_selection_events_fn: |world| {
                    let first_char = first_character(world);
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
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_right),
            },
            ExpectedParams {
                widget_state: WidgetState::CharacterSelect,
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
                widget_state: WidgetState::Ready,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::CharacterSelect,
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
                widget_state: WidgetState::CharacterSelect,
                character_selection_fn: char_bat,
                control_input_event_fn: Some(press_jump),
            },
            ExpectedParams {
                widget_state: WidgetState::Inactive,
                character_selection_fn: char_bat,
                character_selection_events_fn: |_world| {
                    vec![CharacterSelectionEvent::Leave { controller_id: 123 }]
                },
            },
        )
    }

    fn run_test(
        SetupParams {
            widget_state: setup_widget_state,
            character_selection_fn: setup_character_selection_fn,
            control_input_event_fn,
        }: SetupParams,
        ExpectedParams {
            widget_state: expected_widget_state,
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
            .with_setup(move |world| {
                CharacterSelectionWidgetInputSystemData::setup(&mut world.res);

                let setup_character_selection = setup_character_selection_fn(world);
                let entity = widget_entity(world, setup_widget_state, setup_character_selection);
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
                    CharacterSelectionWidget::new(
                        expected_widget_state,
                        expected_character_selection,
                    ),
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

    fn first_character(world: &mut World) -> AssetSlug {
        world
            .read_resource::<CharacterPrefabs>()
            .keys()
            .next()
            .expect("Expected at least one character to be loaded.")
            .clone()
    }

    fn last_character(world: &mut World) -> AssetSlug {
        world
            .read_resource::<CharacterPrefabs>()
            .keys()
            .next_back()
            .expect("Expected at least one character to be loaded.")
            .clone()
    }

    fn char_random(_world: &mut World) -> CharacterSelection {
        CharacterSelection::Random
    }

    fn char_bat(_world: &mut World) -> CharacterSelection {
        CharacterSelection::Id(CHAR_BAT_SLUG.clone())
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
            .with(InputControlled::new(123))
            .build()
    }

    fn assert_widget(world: &mut World, expected: CharacterSelectionWidget) {
        let widget_entity = world.read_resource::<Entity>();

        let widgets = world.read_storage::<CharacterSelectionWidget>();
        let widget = widgets
            .get(*widget_entity)
            .expect("Expected entity to have `CharacterSelectionWidget` component.");

        assert_eq!(expected, *widget);
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
        widget_state: WidgetState,
        character_selection_fn: fn(&mut World) -> CharacterSelection,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
    }

    struct ExpectedParams {
        widget_state: WidgetState,
        character_selection_fn: fn(&mut World) -> CharacterSelection,
        character_selection_events_fn: fn(&mut World) -> Vec<CharacterSelectionEvent>,
    }
}
