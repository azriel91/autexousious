use amethyst::ecs::{Entity, Join};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use character_selection_model::{
    config::{CharacterSelectionEventCommand, SwitchDirection},
    CharacterSelection, CharacterSelectionEvent,
};
use character_selection_ui_model::play::CswStatus;
use log::debug;
use object_type::ObjectType;

use crate::{IrAppEventSender, IrAppEventSenderSystemData};

/// Handles sending `CharacterSelectionEvent`s from input reactions.
#[derive(Debug)]
pub struct IrCharacterSelectionEventSender;

impl IrCharacterSelectionEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        entity: Entity,
        character_selection_event_variant: CharacterSelectionEventCommand,
    ) {
        let character_selection_event = match character_selection_event_variant {
            CharacterSelectionEventCommand::Return => {
                if Self::character_selection_return_preconditions_met(
                    ir_app_event_sender_system_data,
                ) {
                    Some(CharacterSelectionEvent::Return)
                } else {
                    None
                }
            }
            CharacterSelectionEventCommand::Join => {
                ir_app_event_sender_system_data
                    .csw_statuses
                    .insert(entity, CswStatus::CharacterSelect)
                    .expect("Failed to insert `CswStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Join { controller_id })
            }
            CharacterSelectionEventCommand::Leave => {
                ir_app_event_sender_system_data
                    .csw_statuses
                    .insert(entity, CswStatus::Inactive)
                    .expect("Failed to insert `CswStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Leave { controller_id })
            }
            CharacterSelectionEventCommand::Switch(direction) => {
                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::character_selection(
                            ir_app_event_sender_system_data,
                            entity,
                            Some(direction),
                        )
                        .map(|character_selection| (controller_id, character_selection))
                    })
                    .map(
                        |(controller_id, character_selection)| CharacterSelectionEvent::Switch {
                            controller_id,
                            character_selection,
                        },
                    )
            }
            CharacterSelectionEventCommand::Select => {
                ir_app_event_sender_system_data
                    .csw_statuses
                    .insert(entity, CswStatus::Ready)
                    .expect("Failed to insert `CswStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::character_selection(ir_app_event_sender_system_data, entity, None)
                            .map(|character_selection| (controller_id, character_selection))
                    })
                    .map(
                        |(controller_id, character_selection)| CharacterSelectionEvent::Select {
                            controller_id,
                            character_selection,
                        },
                    )
            }
            CharacterSelectionEventCommand::Deselect => {
                ir_app_event_sender_system_data
                    .csw_statuses
                    .insert(entity, CswStatus::CharacterSelect)
                    .expect("Failed to insert `CswStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Deselect { controller_id })
            }
            CharacterSelectionEventCommand::Confirm => {
                if Self::character_selection_confirm_preconditions_met(
                    ir_app_event_sender_system_data,
                    entity,
                ) {
                    Some(CharacterSelectionEvent::Confirm)
                } else {
                    debug!(
                        "Ignoring `CharacterSelectionEvent::Confirm` event as conditions not met."
                    );
                    None
                }
            }
        };

        if let Some(character_selection_event) = character_selection_event {
            ir_app_event_sender_system_data
                .character_selection_ec
                .single_write(character_selection_event);
        }
    }

    fn character_selection_return_preconditions_met(
        IrAppEventSenderSystemData { csw_statuses, .. }: &IrAppEventSenderSystemData,
    ) -> bool {
        // If all widgets are inactive, return to previous `State`.
        csw_statuses
            .join()
            .copied()
            .all(|csw_status| csw_status == CswStatus::Inactive)
    }

    fn character_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            asset_type_mappings,
            character_selections,
            ..
        }: &mut IrAppEventSenderSystemData,
        entity: Entity,
        switch_direction: Option<SwitchDirection>,
    ) -> Option<CharacterSelection> {
        let character_selection = character_selections.get_mut(entity);

        if let Some(character_selection) = character_selection {
            match switch_direction {
                None => Some(*character_selection),
                Some(SwitchDirection::Previous) => {
                    let new_selection =
                        Self::select_previous_character(asset_type_mappings, character_selection);
                    Some(new_selection)
                }
                Some(SwitchDirection::Next) => {
                    let new_selection =
                        Self::select_next_character(asset_type_mappings, character_selection);
                    Some(new_selection)
                }
            }
        } else {
            IrAppEventSender::log_component_missing_error(
                asset_ids,
                asset_id_mappings,
                entity,
                "CharacterSelection",
            );
            None
        }
    }

    fn select_previous_character(
        asset_type_mappings: &AssetTypeMappings,
        character_selection: &mut CharacterSelection,
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
        *character_selection = match *character_selection {
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

        *character_selection
    }

    fn select_next_character(
        asset_type_mappings: &AssetTypeMappings,
        character_selection: &mut CharacterSelection,
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
        *character_selection = match *character_selection {
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

        *character_selection
    }

    fn character_selection_confirm_preconditions_met(
        IrAppEventSenderSystemData { csw_statuses, .. }: &IrAppEventSenderSystemData,
        entity: Entity,
    ) -> bool {
        // If:
        //
        // * All widgets are `Ready` or `Inactive`.
        // * Input was from a `Ready` widget.
        // * There are at least 2 `Ready` widgets`.
        //
        // Then proceed to next `State`.
        let csw_status = csw_statuses.get(entity).copied();

        let all_ready_or_inactive = csw_statuses
            .join()
            .copied()
            .all(|csw_status| csw_status == CswStatus::Ready || csw_status == CswStatus::Inactive);

        let at_least_two_players = csw_statuses
            .join()
            .copied()
            .filter(|csw_status| *csw_status == CswStatus::Ready)
            .count()
            >= 2;

        csw_status == Some(CswStatus::Ready) && at_least_two_players && all_ready_or_inactive
    }
}
