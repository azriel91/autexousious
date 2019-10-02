use amethyst::{
    ecs::{Read, ReadStorage, World, Write, WriteStorage},
    shred::{ResourceId, System, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use character_selection_model::{CharacterSelection, CharacterSelectionEvent};
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use log::debug;
use object_type::ObjectType;

use typename_derive::TypeName;

use crate::{CharacterSelectionWidget, WidgetState};

/// System that processes controller input and generates `CharacterSelectionEvent`s.
///
/// This is not private because consumers may use `CharacterSelectionWidgetInputSystem::type_name()` to
/// specify this as a dependency of another system.
#[derive(Debug, Default, TypeName, new)]
pub struct CharacterSelectionWidgetInputSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
}

/// `CharacterSelectionWidgetInputResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionWidgetInputResources<'s> {
    /// `CharacterSelectionWidget` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_widgets: WriteStorage<'s, CharacterSelectionWidget>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `AssetTypeMappings` assets.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `CharacterSelectionEvent` channel.
    #[derivative(Debug = "ignore")]
    pub character_selection_ec: Write<'s, EventChannel<CharacterSelectionEvent>>,
}

/// `CharacterSelectionWidgetInputSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionWidgetInputSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `CharacterSelectionWidgetInputResources`.
    pub character_selection_widget_input_resources: CharacterSelectionWidgetInputResources<'s>,
}

impl CharacterSelectionWidgetInputSystem {
    fn select_previous_character(
        asset_type_mappings: &AssetTypeMappings,
        widget: &mut CharacterSelectionWidget,
    ) -> CharacterSelection {
        let first_character_id = asset_type_mappings
            .iter_ids(&AssetType::Object(ObjectType::Character))
            .copied()
            .next()
            .expect("Expected at least one character to be loaded.");
        let last_character_id = asset_type_mappings
            .iter_ids(&AssetType::Object(ObjectType::Character))
            .copied()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(character_id) => {
                if character_id == first_character_id {
                    CharacterSelection::Random
                } else {
                    let next_character = asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Character))
                        .copied()
                        .rev()
                        .skip_while(|id| id != &character_id)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character)
                    } else {
                        CharacterSelection::Random
                    }
                }
            }
            CharacterSelection::Random => CharacterSelection::Id(last_character_id),
        };
        widget.selection
    }

    fn select_next_character(
        asset_type_mappings: &AssetTypeMappings,
        widget: &mut CharacterSelectionWidget,
    ) -> CharacterSelection {
        let first_character_id = asset_type_mappings
            .iter_ids(&AssetType::Object(ObjectType::Character))
            .copied()
            .next()
            .expect("Expected at least one character to be loaded.");
        let last_character_id = asset_type_mappings
            .iter_ids(&AssetType::Object(ObjectType::Character))
            .copied()
            .next_back()
            .expect("Expected at least one character to be loaded.");
        widget.selection = match widget.selection {
            CharacterSelection::Id(character_id) => {
                if character_id == last_character_id {
                    CharacterSelection::Random
                } else {
                    let next_character = asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Character))
                        .copied()
                        .skip_while(|id| id != &character_id)
                        .nth(1); // skip current selection

                    if let Some(next_character) = next_character {
                        CharacterSelection::Id(next_character)
                    } else {
                        CharacterSelection::Random
                    }
                }
            }
            CharacterSelection::Random => CharacterSelection::Id(first_character_id),
        };
        widget.selection
    }

    fn handle_event(
        CharacterSelectionWidgetInputResources {
            ref mut character_selection_widgets,
            ref input_controlleds,
            ref asset_type_mappings,
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
                        &asset_type_mappings,
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
        asset_type_mappings: &AssetTypeMappings,
        character_selection_ec: &mut EventChannel<CharacterSelectionEvent>,
        character_selection_widget: &mut CharacterSelectionWidget,
        input_controlled: InputControlled,
        axis_move_event_data: AxisMoveEventData,
    ) {
        let character_selection =
            match (character_selection_widget.state, axis_move_event_data.axis) {
                (WidgetState::CharacterSelect, Axis::X) if axis_move_event_data.value < 0. => {
                    Some(Self::select_previous_character(
                        asset_type_mappings,
                        character_selection_widget,
                    ))
                }
                (WidgetState::CharacterSelect, Axis::X) if axis_move_event_data.value > 0. => Some(
                    Self::select_next_character(asset_type_mappings, character_selection_widget),
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
                    character_selection: character_selection_widget.selection,
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
        CharacterSelectionWidgetInputSystemData {
            control_input_ec,
            mut character_selection_widget_input_resources,
        }: Self::SystemData,
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

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
