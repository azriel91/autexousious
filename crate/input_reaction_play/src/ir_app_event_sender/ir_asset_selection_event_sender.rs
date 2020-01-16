use std::convert::TryFrom;

use amethyst::ecs::{Entity, Join};
use asset_model::{
    config::{AssetSelectionEventCommand, AssetSwitch, AssetType},
    loaded::AssetTypeMappings,
    play::{AssetSelection, AssetSelectionEvent},
};
use asset_ui_model::play::AshStatus;
use log::debug;
use object_type::ObjectType;

use crate::{IrAppEventSender, IrAppEventSenderSystemData};

/// Handles sending `AssetSelectionEvent`s from input reactions.
#[derive(Debug)]
pub struct IrAssetSelectionEventSender;

impl IrAssetSelectionEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        entity: Entity,
        asset_selection_event_variant: AssetSelectionEventCommand,
    ) {
        let asset_selection_event = match asset_selection_event_variant {
            AssetSelectionEventCommand::Return => {
                if Self::asset_selection_return_preconditions_met(ir_app_event_sender_system_data) {
                    Some(AssetSelectionEvent::Return)
                } else {
                    None
                }
            }
            AssetSelectionEventCommand::Join => {
                ir_app_event_sender_system_data
                    .ash_statuses
                    .insert(entity, AshStatus::AssetSelect)
                    .expect("Failed to insert `AshStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| AssetSelectionEvent::Join { controller_id })
            }
            AssetSelectionEventCommand::Leave => {
                ir_app_event_sender_system_data
                    .ash_statuses
                    .insert(entity, AshStatus::Inactive)
                    .expect("Failed to insert `AshStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| AssetSelectionEvent::Leave { controller_id })
            }
            AssetSelectionEventCommand::Switch(direction) => {
                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::asset_selection(
                            ir_app_event_sender_system_data,
                            entity,
                            Some(direction),
                        )
                        .map(|asset_selection| (controller_id, asset_selection))
                    })
                    .map(
                        |(controller_id, asset_selection)| AssetSelectionEvent::Switch {
                            controller_id,
                            asset_selection,
                        },
                    )
            }
            AssetSelectionEventCommand::Select => {
                ir_app_event_sender_system_data
                    .ash_statuses
                    .insert(entity, AshStatus::Ready)
                    .expect("Failed to insert `AshStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::asset_selection(ir_app_event_sender_system_data, entity, None)
                            .map(|asset_selection| (controller_id, asset_selection))
                    })
                    .map(
                        |(controller_id, asset_selection)| AssetSelectionEvent::Select {
                            controller_id,
                            asset_selection,
                        },
                    )
            }
            AssetSelectionEventCommand::Deselect => {
                ir_app_event_sender_system_data
                    .ash_statuses
                    .insert(entity, AshStatus::AssetSelect)
                    .expect("Failed to insert `AshStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| AssetSelectionEvent::Deselect { controller_id })
            }
            AssetSelectionEventCommand::Confirm => {
                if Self::asset_selection_confirm_preconditions_met(
                    ir_app_event_sender_system_data,
                    entity,
                ) {
                    Some(AssetSelectionEvent::Confirm)
                } else {
                    debug!("Ignoring `AssetSelectionEvent::Confirm` event as conditions not met.");
                    None
                }
            }
        };

        if let Some(asset_selection_event) = asset_selection_event {
            ir_app_event_sender_system_data
                .asset_selection_ec
                .single_write(asset_selection_event);
        }
    }

    fn asset_selection_return_preconditions_met(
        IrAppEventSenderSystemData { ash_statuses, .. }: &IrAppEventSenderSystemData,
    ) -> bool {
        // If all widgets are inactive, return to previous `State`.
        ash_statuses
            .join()
            .copied()
            .all(|ash_status| ash_status == AshStatus::Inactive)
    }

    fn asset_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            asset_type_mappings,
            asset_selections,
            ..
        }: &mut IrAppEventSenderSystemData,
        entity: Entity,
        switch_direction: Option<AssetSwitch>,
    ) -> Option<AssetSelection> {
        let asset_selection = asset_selections.get_mut(entity);

        if let Some(asset_selection) = asset_selection {
            match switch_direction {
                None => Some(*asset_selection),
                Some(AssetSwitch::Previous) => {
                    let new_selection =
                        Self::switch_asset(asset_type_mappings, asset_selection, -1);
                    Some(new_selection)
                }
                Some(AssetSwitch::Next) => {
                    let new_selection = Self::switch_asset(asset_type_mappings, asset_selection, 1);
                    Some(new_selection)
                }
                Some(AssetSwitch::Skip(n)) => {
                    let new_selection =
                        Self::switch_asset(asset_type_mappings, asset_selection, isize::from(n));
                    Some(new_selection)
                }
            }
        } else {
            IrAppEventSender::log_component_missing_error(
                asset_ids,
                asset_id_mappings,
                entity,
                "AssetSelection",
            );
            None
        }
    }

    fn switch_asset(
        asset_type_mappings: &AssetTypeMappings,
        asset_selection: &mut AssetSelection,
        n: isize,
    ) -> AssetSelection {
        let reverse = n < 0;
        let n = usize::try_from(n.wrapping_abs()).expect("Failed to convert `n` into `usize`.");
        let n = n + 1; // skip current selection

        let new_selection = {
            let placeholder = Vec::new();
            let asset_ids = asset_type_mappings
                .get_ids(AssetType::Object(ObjectType::Character)) // TODO: Generic
                .unwrap_or(&placeholder);
            let mut asset_selections = Vec::with_capacity(asset_ids.len() + 1);
            asset_selections.extend(asset_ids.into_iter().copied().map(AssetSelection::Id));

            if reverse {
                asset_selections
                    .into_iter()
                    .rev()
                    .skip_while(|selection| selection != asset_selection)
                    .nth(n)
            } else {
                asset_selections
                    .into_iter()
                    .skip_while(|selection| selection != asset_selection)
                    .nth(n)
            }
        }
        .expect("Expected at least one asset to be loaded.");

        *asset_selection = new_selection;
        new_selection
    }

    fn asset_selection_confirm_preconditions_met(
        IrAppEventSenderSystemData { ash_statuses, .. }: &IrAppEventSenderSystemData,
        entity: Entity,
    ) -> bool {
        // If:
        //
        // * All widgets are `Ready` or `Inactive`.
        // * Input was from a `Ready` widget.
        // * There are at least 2 `Ready` widgets`.
        //
        // Then proceed to next `State`.
        let ash_status = ash_statuses.get(entity).copied();

        let all_ready_or_inactive = ash_statuses
            .join()
            .copied()
            .all(|ash_status| ash_status == AshStatus::Ready || ash_status == AshStatus::Inactive);

        let at_least_two_players = ash_statuses
            .join()
            .copied()
            .filter(|ash_status| *ash_status == AshStatus::Ready)
            .count()
            >= 2;

        ash_status == Some(AshStatus::Ready) && at_least_two_players && all_ready_or_inactive
    }
}
