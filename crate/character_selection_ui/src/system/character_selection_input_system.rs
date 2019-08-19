use amethyst::{
    ecs::{Join, Read, ReadStorage, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use character_selection_model::CharacterSelectionEvent;
use derivative::Derivative;
use derive_new::new;
use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
use log::debug;
use typename_derive::TypeName;

use crate::{CharacterSelectionWidget, WidgetState};

/// Processes controller input to decide when the character selection screen should transition.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterSelectionInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub(crate) struct CharacterSelectionInputSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `CharacterSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_widgets: ReadStorage<'s, CharacterSelectionWidget>,
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Write<'s, EventChannel<CharacterSelectionEvent>>,
}

impl CharacterSelectionInputSystem {
    fn handle_control_action_event(
        character_selection_widgets: &ReadStorage<'_, CharacterSelectionWidget>,
        character_selection_ec: &mut EventChannel<CharacterSelectionEvent>,
        control_action_event_data: ControlActionEventData,
    ) {
        let character_selection_event = match control_action_event_data.control_action {
            ControlAction::Jump => {
                // If all widgets are inactive, return to previous `State`.
                let all_inactive =
                    character_selection_widgets
                        .join()
                        .all(|character_selection_widget| {
                            character_selection_widget.state == WidgetState::Inactive
                        });
                if all_inactive {
                    Some(CharacterSelectionEvent::Return)
                } else {
                    None
                }
            }
            ControlAction::Attack => {
                // If:
                //
                // * All widgets are `Ready` or `Inactive`.
                // * Input was from a `Ready` widget.
                // * There are at least 2 `Ready` widgets`.
                //
                // Then proceed to next `State`.
                let character_selection_widget =
                    character_selection_widgets.get(control_action_event_data.entity);
                if let Some(character_selection_widget) = character_selection_widget {
                    let all_ready_or_inactive =
                        character_selection_widgets
                            .join()
                            .all(|character_selection_widget| {
                                character_selection_widget.state == WidgetState::Ready
                                    || character_selection_widget.state == WidgetState::Inactive
                            });

                    let at_least_two_players = character_selection_widgets
                        .join()
                        .filter(|character_selection_widget| {
                            character_selection_widget.state == WidgetState::Ready
                        })
                        .count()
                        >= 2;

                    if character_selection_widget.state == WidgetState::Ready
                        && at_least_two_players
                        && all_ready_or_inactive
                    {
                        Some(CharacterSelectionEvent::Confirm)
                    } else {
                        None
                    }
                } else {
                    None
                }
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

impl<'s> System<'s> for CharacterSelectionInputSystem {
    type SystemData = CharacterSelectionInputSystemData<'s>;

    fn run(
        &mut self,
        CharacterSelectionInputSystemData {
            control_input_ec,
            character_selection_widgets,
            mut character_selection_ec,
        }: Self::SystemData,
    ) {
        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                if let ControlInputEvent::ControlActionPress(control_action_event_data) = ev {
                    Self::handle_control_action_event(
                        &character_selection_widgets,
                        &mut character_selection_ec,
                        *control_action_event_data,
                    )
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        ecs::{Builder, Entity, World},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application_test_support::AutexousiousApplication;
    use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
    use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
    use typename::TypeName;

    use super::{CharacterSelectionInputSystem, CharacterSelectionInputSystemData};
    use crate::{CharacterSelectionWidget, WidgetState};

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
                &CharacterSelectionInputSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_setup(move |world| {
                CharacterSelectionInputSystemData::setup(&mut world.res);

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
            .run_isolated()
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
