use std::convert::TryFrom;

use amethyst::ecs::{Entity, Join};
use asset_model::{
    config::{AssetSelectionEventCommand, AssetSwitch, AssetType},
    loaded::AssetTypeMappings,
    play::{AssetSelection, AssetSelectionEvent},
};
use asset_ui_model::play::AssetSelectionStatus;
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
        // For `CharacterSelectionWidget` entities, `entity` is the `CswMain` entity.
        //
        // For `AssetSelectionHighlightMain` entities, `entity` that sends the event is not the
        // `AssetSelectionHighlightMain` entity, but its `TargetObject` is.
        let ash_entity = ir_app_event_sender_system_data
            .target_objects
            .get(entity)
            .map(|target_object| target_object.entity)
            .unwrap_or(entity);

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
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::InProgress)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, ash_entity)
                    .map(|controller_id| AssetSelectionEvent::Join { controller_id })
            }
            AssetSelectionEventCommand::Leave => {
                ir_app_event_sender_system_data
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::Inactive)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, ash_entity)
                    .map(|controller_id| AssetSelectionEvent::Leave { controller_id })
            }
            AssetSelectionEventCommand::Switch(direction) => {
                IrAppEventSender::controller_id(ir_app_event_sender_system_data, ash_entity)
                    .and_then(|controller_id| {
                        Self::asset_selection(
                            ir_app_event_sender_system_data,
                            ash_entity,
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
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::Ready)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, ash_entity)
                    .and_then(|controller_id| {
                        Self::asset_selection(ir_app_event_sender_system_data, ash_entity, None)
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
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::InProgress)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                IrAppEventSender::controller_id(ir_app_event_sender_system_data, ash_entity)
                    .map(|controller_id| AssetSelectionEvent::Deselect { controller_id })
            }
            AssetSelectionEventCommand::Confirm => {
                if Self::asset_selection_confirm_preconditions_met(
                    ir_app_event_sender_system_data,
                    ash_entity,
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
        IrAppEventSenderSystemData {
            asset_selection_statuses,
            ..
        }: &IrAppEventSenderSystemData,
    ) -> bool {
        // If all widgets are inactive, return to previous `State`.
        asset_selection_statuses
            .join()
            .copied()
            .all(|asset_selection_status| asset_selection_status == AssetSelectionStatus::Inactive)
    }

    fn asset_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            asset_type_mappings,
            target_objects,
            asset_selections,
            ..
        }: &mut IrAppEventSenderSystemData,
        ash_entity: Entity,
        switch_direction: Option<AssetSwitch>,
    ) -> Option<AssetSelection> {
        // Look up the `TargetObject` whose entity is the `AssetSelectionCell`
        // Then lookup the `AssetSelection` for that entity.
        let asset_selection = target_objects
            .get(ash_entity)
            // The `TargetObject` of the `AssetSelectionHighlightMain` entity is the
            // `AssetSelectionCell`.
            .and_then(|target_object| asset_selections.get(target_object.entity).copied());

        if let Some(asset_selection) = asset_selection {
            match switch_direction {
                None => Some(asset_selection),
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
                ash_entity,
                "AssetSelection",
            );
            None
        }
    }

    fn switch_asset(
        asset_type_mappings: &AssetTypeMappings,
        asset_selection: AssetSelection,
        n: isize,
    ) -> AssetSelection {
        let reverse = n < 0;
        let n = usize::try_from(n.wrapping_abs()).expect("Failed to convert `n` into `usize`.");
        let n = n + 1; // skip current selection

        {
            let placeholder = Vec::new();
            let asset_ids = asset_type_mappings
                .get_ids(AssetType::Object(ObjectType::Character)) // TODO: Generic
                .unwrap_or(&placeholder);
            let mut asset_selections = Vec::with_capacity(asset_ids.len() + 1);
            asset_selections.push(AssetSelection::Random);
            asset_selections.extend(asset_ids.into_iter().copied().map(AssetSelection::Id));

            if reverse {
                asset_selections
                    .into_iter()
                    .rev()
                    .cycle()
                    .skip_while(|selection| *selection != asset_selection)
                    .nth(n)
            } else {
                asset_selections
                    .into_iter()
                    .cycle()
                    .skip_while(|selection| *selection != asset_selection)
                    .nth(n)
            }
        }
        .expect("Expected at least one asset to be loaded.")
    }

    fn asset_selection_confirm_preconditions_met(
        IrAppEventSenderSystemData {
            asset_selection_statuses,
            ..
        }: &IrAppEventSenderSystemData,
        ash_entity: Entity,
    ) -> bool {
        // If:
        //
        // * All widgets are `Ready` or `Inactive`.
        // * Input was from a `Ready` widget.
        // * There are at least 2 `Ready` widgets`.
        //
        // Then proceed to next `State`.
        let asset_selection_status = asset_selection_statuses.get(ash_entity).copied();

        let all_ready_or_inactive =
            asset_selection_statuses
                .join()
                .copied()
                .all(|asset_selection_status| {
                    asset_selection_status == AssetSelectionStatus::Ready
                        || asset_selection_status == AssetSelectionStatus::Inactive
                });

        let at_least_two_players = asset_selection_statuses
            .join()
            .copied()
            .filter(|asset_selection_status| *asset_selection_status == AssetSelectionStatus::Ready)
            .count()
            >= 2;

        asset_selection_status == Some(AssetSelectionStatus::Ready)
            && at_least_two_players
            && all_ready_or_inactive
    }
}
