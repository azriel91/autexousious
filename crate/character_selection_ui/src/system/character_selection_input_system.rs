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

use crate::CharacterSelectionWidgetState;

/// Processes controller input to decide when the character selection screen should transition.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

/// `CharacterSelectionInputSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionInputSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `CharacterSelectionWidgetState` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_widget_states: ReadStorage<'s, CharacterSelectionWidgetState>,
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Write<'s, EventChannel<CharacterSelectionEvent>>,
}

impl CharacterSelectionInputSystem {
    fn handle_control_action_event(
        character_selection_widget_states: &ReadStorage<'_, CharacterSelectionWidgetState>,
        character_selection_ec: &mut EventChannel<CharacterSelectionEvent>,
        control_action_event_data: ControlActionEventData,
    ) {
        if character_selection_widget_states.is_empty() {
            return;
        }
        let character_selection_event = match control_action_event_data.control_action {
            ControlAction::Jump => {
                // If all widgets are inactive, return to previous `State`.
                let all_inactive = character_selection_widget_states.join().copied().all(
                    |character_selection_widget_state| {
                        character_selection_widget_state == CharacterSelectionWidgetState::Inactive
                    },
                );
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
                let character_selection_widget_state = character_selection_widget_states
                    .get(control_action_event_data.entity)
                    .copied();
                if let Some(character_selection_widget_state) = character_selection_widget_state {
                    let all_ready_or_inactive = character_selection_widget_states
                        .join()
                        .copied()
                        .all(|character_selection_widget_state| {
                            character_selection_widget_state == CharacterSelectionWidgetState::Ready
                                || character_selection_widget_state
                                    == CharacterSelectionWidgetState::Inactive
                        });

                    let at_least_two_players = character_selection_widget_states
                        .join()
                        .copied()
                        .filter(|character_selection_widget_state| {
                            *character_selection_widget_state
                                == CharacterSelectionWidgetState::Ready
                        })
                        .count()
                        >= 2;

                    if character_selection_widget_state == CharacterSelectionWidgetState::Ready
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
            character_selection_widget_states,
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
                        &character_selection_widget_states,
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
