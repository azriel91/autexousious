use std::convert::TryFrom;

use amethyst::ecs::{Entity, Join};
use asset_model::{
    config::{AssetSelectionEventCommand, AssetSwitch, AssetType},
    loaded::AssetTypeMappings,
    play::{AssetSelection, AssetSelectionEvent},
};
use asset_ui_model::play::AssetSelectionStatus;
use game_input_model::ControllerId;
use log::{debug, warn};
use object_type::ObjectType;
use state_registry::StateId;

use crate::{IrAppEventSender, IrAppEventSenderSystemData};

/// Handles sending `AssetSelectionEvent`s from input reactions.
#[derive(Debug)]
pub struct IrAssetSelectionEventSender;

impl IrAssetSelectionEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        controller_id: ControllerId,
        entity: Entity,
        asset_selection_event_variant: AssetSelectionEventCommand,
    ) {
        // For `AssetPreviewWidget` entities, `entity` is the `ApwMain` entity.
        //
        // For `AssetSelectionHighlightMain` entities, `entity` that sends the event is not the
        // `AssetSelectionHighlightMain` entity, but its `TargetObject` is.
        let target_object_entity = ir_app_event_sender_system_data
            .target_objects
            .get(entity)
            .map(|target_object| target_object.entity);
        let ash_entity = if let Some(target_object_entity) = target_object_entity {
            if ir_app_event_sender_system_data
                .asset_selection_highlight_mains
                .contains(target_object_entity)
            {
                target_object_entity
            } else {
                entity
            }
        } else {
            entity
        };

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

                Some(AssetSelectionEvent::Join {
                    entity: Some(ash_entity),
                    controller_id,
                })
            }
            AssetSelectionEventCommand::Leave => {
                ir_app_event_sender_system_data
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::Inactive)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                Some(AssetSelectionEvent::Leave {
                    entity: Some(ash_entity),
                    controller_id,
                })
            }
            AssetSelectionEventCommand::Switch(direction) => {
                Self::asset_selection(ir_app_event_sender_system_data, ash_entity, Some(direction))
                    .map(|asset_selection| AssetSelectionEvent::Switch {
                        entity: Some(ash_entity),
                        controller_id,
                        asset_selection,
                    })
            }
            AssetSelectionEventCommand::Select => {
                ir_app_event_sender_system_data
                    .asset_selection_statuses
                    .insert(ash_entity, AssetSelectionStatus::Ready)
                    .expect("Failed to insert `AssetSelectionStatus` component.");

                Self::asset_selection(ir_app_event_sender_system_data, ash_entity, None).map(
                    |asset_selection| AssetSelectionEvent::Select {
                        entity: Some(ash_entity),
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

                Some(AssetSelectionEvent::Deselect {
                    entity: Some(ash_entity),
                    controller_id,
                })
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
            state_id,
            ..
        }: &IrAppEventSenderSystemData,
    ) -> bool {
        let state_id = **state_id;
        match state_id {
            StateId::CharacterSelection => {
                // If all widgets are inactive, return to previous `State`.
                asset_selection_statuses
                    .join()
                    .copied()
                    .all(|asset_selection_status| {
                        asset_selection_status == AssetSelectionStatus::Inactive
                    })
            }
            StateId::MapSelection => true,
            _ => {
                warn!("`AssetSelection` is not supported during `{:?}`.", state_id);
                false
            }
        }
    }

    fn asset_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            asset_type_mappings,
            state_id,
            target_objects,
            asset_selections,
            asset_selection_highlight_mains,
            ..
        }: &mut IrAppEventSenderSystemData,
        ash_entity: Entity,
        switch_direction: Option<AssetSwitch>,
    ) -> Option<AssetSelection> {
        // Look up the `TargetObject` whose entity is the `AssetSelectionCell`
        // Then lookup the `AssetSelection` for that entity.
        let asset_selection_entity = if asset_selection_highlight_mains.contains(ash_entity) {
            target_objects
                .get(ash_entity)
                .map(|target_object| target_object.entity)
                .expect(
                    "Expected `AssetSelectionHighlightMain` entity to have `TargetObject` \
                    component.",
                )
        } else {
            // `ApwMain` entities directly send events.
            // Other purpose entities likely also fallback to the same behaviour.
            ash_entity
        };
        // The `TargetObject` of the `AssetSelectionHighlightMain` entity is the
        // `AssetSelectionCell`.
        let asset_selection = asset_selections.get(asset_selection_entity).copied();

        let state_id = **state_id;
        if let Some(asset_selection) = asset_selection {
            match switch_direction {
                None => Some(asset_selection),
                Some(AssetSwitch::Previous) => {
                    let new_selection =
                        Self::switch_asset(asset_type_mappings, state_id, asset_selection, -1);
                    Some(new_selection)
                }
                Some(AssetSwitch::Next) => {
                    let new_selection =
                        Self::switch_asset(asset_type_mappings, state_id, asset_selection, 1);
                    Some(new_selection)
                }
                Some(AssetSwitch::Skip(n)) => {
                    let new_selection = Self::switch_asset(
                        asset_type_mappings,
                        state_id,
                        asset_selection,
                        isize::from(n),
                    );
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
        state_id: StateId,
        asset_selection: AssetSelection,
        n: isize,
    ) -> AssetSelection {
        let reverse = n < 0;
        let n = usize::try_from(n.wrapping_abs()).expect("Failed to convert `n` into `usize`.");

        {
            let placeholder = Vec::new();

            // Determine what kind of asset we are selecting:
            let asset_type = match state_id {
                StateId::CharacterSelection => AssetType::Object(ObjectType::Character),
                StateId::MapSelection => AssetType::Map,
                _ => {
                    warn!("`AssetSelection` is not supported during `{:?}`.", state_id);
                    return asset_selection;
                }
            };
            let asset_ids = asset_type_mappings
                .get_ids(asset_type)
                .unwrap_or(&placeholder);
            let mut asset_selections = Vec::with_capacity(asset_ids.len() + 1);
            asset_selections.push(AssetSelection::Random);
            asset_selections.extend(asset_ids.iter().copied().map(AssetSelection::Id));

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
        ir_app_event_sender_system_data: &IrAppEventSenderSystemData,
        ash_entity: Entity,
    ) -> bool {
        let state_id = *ir_app_event_sender_system_data.state_id;
        match state_id {
            StateId::CharacterSelection => Self::character_selection_confirm_preconditions_met(
                ir_app_event_sender_system_data,
                ash_entity,
            ),
            StateId::MapSelection => true,
            _ => {
                warn!("`AssetSelection` is not supported during `{:?}`.", state_id);
                false
            }
        }
    }

    fn character_selection_confirm_preconditions_met(
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
