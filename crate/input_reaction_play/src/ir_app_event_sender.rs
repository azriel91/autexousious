use amethyst::ecs::{Entity, ReadStorage};
use asset_model::loaded::{AssetId, AssetIdMappings};
use character_selection_model::{
    CharacterSelection, CharacterSelectionEvent, CharacterSelectionEventVariant,
};
use control_settings_model::ControlSettingsEvent;
use game_input::InputControlled;
use game_input_model::ControllerId;
use game_mode_selection_model::{GameModeSelectionEvent, GameModeSelectionEventArgs};
use game_play_model::{GamePlayEvent, GamePlayEventArgs};
use input_reaction_model::config::InputReactionAppEvent;
use log::error;
use map_selection_model::{MapSelection, MapSelectionEvent, MapSelectionEventVariant};

use crate::IrAppEventSenderSystemData;

/// Maps `InputReactionAppEvent`s to the actual event and sends it to its event channel.
#[derive(Debug)]
pub struct IrAppEventSender;

impl IrAppEventSender {
    /// Maps `InputReactionAppEvent`s to the actual event and sends it to its event channel.
    ///
    /// If necessary, this involves looking up additional information from resources to populate the
    /// actual event variant's fields.
    ///
    /// # Parameters
    ///
    /// * `entity`: Entity that the input reaction is sourced from.
    /// * `event`: `AppEvent` command variant to send.
    pub fn send(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        entity: Entity,
        event: InputReactionAppEvent,
    ) {
        match event {
            InputReactionAppEvent::CharacterSelection(character_selection_event_variant) => {
                Self::handle_character_selection_event(
                    ir_app_event_sender_system_data,
                    entity,
                    character_selection_event_variant,
                );
            }
            InputReactionAppEvent::ControlSettings(control_settings_event) => {
                Self::handle_control_settings_event(
                    ir_app_event_sender_system_data,
                    control_settings_event,
                );
            }
            InputReactionAppEvent::GameModeSelection(game_mode_selection_event_args) => {
                Self::handle_game_mode_selection_event(
                    ir_app_event_sender_system_data,
                    game_mode_selection_event_args,
                );
            }
            InputReactionAppEvent::GamePlay(game_play_event_args) => {
                Self::handle_game_play_event(ir_app_event_sender_system_data, game_play_event_args);
            }
            InputReactionAppEvent::MapSelection(map_selection_event_variant) => {
                Self::handle_map_selection_event(
                    ir_app_event_sender_system_data,
                    entity,
                    map_selection_event_variant,
                );
            }
        }
    }

    fn handle_character_selection_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        entity: Entity,
        character_selection_event_variant: CharacterSelectionEventVariant,
    ) {
        let character_selection_event = match character_selection_event_variant {
            CharacterSelectionEventVariant::Return => Some(CharacterSelectionEvent::Return),
            CharacterSelectionEventVariant::Join => {
                Self::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Join { controller_id })
            }
            CharacterSelectionEventVariant::Leave => {
                Self::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Leave { controller_id })
            }
            CharacterSelectionEventVariant::Switch => {
                Self::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::character_selection(ir_app_event_sender_system_data, entity)
                            .map(|character_selection| (controller_id, character_selection))
                    })
                    .map(
                        |(controller_id, character_selection)| CharacterSelectionEvent::Switch {
                            controller_id,
                            character_selection,
                        },
                    )
            }
            CharacterSelectionEventVariant::Select => {
                Self::controller_id(ir_app_event_sender_system_data, entity)
                    .and_then(|controller_id| {
                        Self::character_selection(ir_app_event_sender_system_data, entity)
                            .map(|character_selection| (controller_id, character_selection))
                    })
                    .map(
                        |(controller_id, character_selection)| CharacterSelectionEvent::Select {
                            controller_id,
                            character_selection,
                        },
                    )
            }
            CharacterSelectionEventVariant::Deselect => {
                Self::controller_id(ir_app_event_sender_system_data, entity)
                    .map(|controller_id| CharacterSelectionEvent::Deselect { controller_id })
            }
            CharacterSelectionEventVariant::Confirm => Some(CharacterSelectionEvent::Confirm),
        };

        if let Some(character_selection_event) = character_selection_event {
            ir_app_event_sender_system_data
                .character_selection_ec
                .single_write(character_selection_event);
        }
    }

    fn controller_id(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            input_controlleds,
            ..
        }: &IrAppEventSenderSystemData,
        entity: Entity,
    ) -> Option<ControllerId> {
        let input_controlled = input_controlleds.get(entity).copied();

        if let Some(InputControlled { controller_id }) = input_controlled {
            Some(controller_id)
        } else {
            Self::log_component_missing_error(
                asset_ids,
                asset_id_mappings,
                entity,
                "InputControlled",
            );
            None
        }
    }

    fn character_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            character_selections,
            ..
        }: &IrAppEventSenderSystemData,
        entity: Entity,
    ) -> Option<CharacterSelection> {
        let character_selection = character_selections.get(entity).copied();

        if let Some(character_selection) = character_selection {
            Some(character_selection)
        } else {
            Self::log_component_missing_error(
                asset_ids,
                asset_id_mappings,
                entity,
                "CharacterSelection",
            );
            None
        }
    }

    fn handle_control_settings_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        control_settings_event: ControlSettingsEvent,
    ) {
        ir_app_event_sender_system_data
            .control_settings_ec
            .single_write(control_settings_event);
    }

    fn handle_game_mode_selection_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        game_mode_selection_event_args: GameModeSelectionEventArgs,
    ) {
        let game_mode_selection_event = match game_mode_selection_event_args {
            GameModeSelectionEventArgs::Select { index } => GameModeSelectionEvent::Select(index),
            GameModeSelectionEventArgs::Close => GameModeSelectionEvent::Close,
        };

        ir_app_event_sender_system_data
            .game_mode_selection_ec
            .single_write(game_mode_selection_event);
    }

    fn handle_game_play_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        game_play_event_args: GamePlayEventArgs,
    ) {
        let game_play_event = match game_play_event_args {
            GamePlayEventArgs::Return => GamePlayEvent::Return,
            GamePlayEventArgs::Restart => GamePlayEvent::Restart,
            GamePlayEventArgs::Pause => GamePlayEvent::Pause,
            GamePlayEventArgs::Resume => GamePlayEvent::Resume,
            GamePlayEventArgs::End => GamePlayEvent::End,
            GamePlayEventArgs::EndStats => GamePlayEvent::EndStats,
        };

        ir_app_event_sender_system_data
            .game_play_ec
            .single_write(game_play_event);
    }

    fn handle_map_selection_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        entity: Entity,
        map_selection_event_variant: MapSelectionEventVariant,
    ) {
        let map_selection_event = match map_selection_event_variant {
            MapSelectionEventVariant::Return => Some(MapSelectionEvent::Return),
            MapSelectionEventVariant::Switch => {
                Self::map_selection(ir_app_event_sender_system_data, entity)
                    .map(|map_selection| MapSelectionEvent::Switch { map_selection })
            }
            MapSelectionEventVariant::Select => {
                Self::map_selection(ir_app_event_sender_system_data, entity)
                    .map(|map_selection| MapSelectionEvent::Select { map_selection })
            }
            MapSelectionEventVariant::Deselect => Some(MapSelectionEvent::Deselect),
            MapSelectionEventVariant::Confirm => Some(MapSelectionEvent::Confirm),
        };

        if let Some(map_selection_event) = map_selection_event {
            ir_app_event_sender_system_data
                .map_selection_ec
                .single_write(map_selection_event);
        }
    }

    fn map_selection(
        IrAppEventSenderSystemData {
            asset_ids,
            asset_id_mappings,
            map_selections,
            ..
        }: &IrAppEventSenderSystemData,
        entity: Entity,
    ) -> Option<MapSelection> {
        let map_selection = map_selections.get(entity).copied();

        if let Some(map_selection) = map_selection {
            Some(map_selection)
        } else {
            Self::log_component_missing_error(asset_ids, asset_id_mappings, entity, "MapSelection");
            None
        }
    }

    fn log_component_missing_error(
        asset_ids: &ReadStorage<'_, AssetId>,
        asset_id_mappings: &AssetIdMappings,
        entity: Entity,
        component_type: &'static str,
    ) {
        let asset_id = asset_ids.get(entity).copied();

        if let Some(asset_id) = asset_id {
            let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
                panic!(
                    "Expected `AssetSlug` to exist for `AssetId`: `{:?}`.",
                    asset_id
                )
            });

            // TODO: look up `AssetType` from `AssetTypeMappings`, and based on the
            // `SequenceName` for `AssetType`, look up
            // `SequenceIdMappings<SeqName>`, then get the `SequenceNameString`
            // based on the `SequenceId` that this entity has.
            //
            // Also, probably better done in a dedicated error reporting system.
            error!(
                "Failed to retrieve `{}` component for entity with sequence from asset: `{}`",
                component_type, asset_slug
            );
        } else {
            error!(
                "Failed to retrieve `{}` component for entity.",
                component_type
            );
        }
    }
}
