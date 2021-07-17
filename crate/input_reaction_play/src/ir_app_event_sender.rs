use amethyst::ecs::{Entity, ReadStorage};
use asset_model::loaded::{AssetId, AssetIdMappings};
use control_settings_model::ControlSettingsEvent;
use game_input_model::config::ControllerId;
use game_mode_selection_model::{GameModeSelectionEvent, GameModeSelectionEventArgs};
use game_play_model::{GamePlayEvent, GamePlayEventArgs};
use input_reaction_model::config::InputReactionAppEvent;
use log::{debug, error};
use network_mode_selection_model::{NetworkModeSelectionEvent, NetworkModeSelectionEventArgs};

use crate::IrAppEventSenderSystemData;

use self::{
    ir_asset_selection_event_sender::IrAssetSelectionEventSender,
    ir_session_host_event_sender::IrSessionHostEventSender,
    ir_session_join_event_sender::IrSessionJoinEventSender,
    ir_session_lobby_event_sender::IrSessionLobbyEventSender,
};

mod ir_asset_selection_event_sender;
mod ir_session_host_event_sender;
mod ir_session_join_event_sender;
mod ir_session_lobby_event_sender;

/// Maps `InputReactionAppEvent`s to the actual event and sends it to its event
/// channel.
#[derive(Debug)]
pub struct IrAppEventSender;

impl IrAppEventSender {
    /// Maps `InputReactionAppEvent`s to the actual event and sends it to its
    /// event channel.
    ///
    /// If necessary, this involves looking up additional information from
    /// resources to populate the actual event variant's fields.
    ///
    /// # Parameters
    ///
    /// * `entity`: Entity that the input reaction is sourced from.
    /// * `event`: `AppEvent` command variant to send.
    pub fn send(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        controller_id: Option<ControllerId>,
        entity: Entity,
        event: InputReactionAppEvent,
    ) {
        debug!("Sending {:?}.", event);
        match event {
            InputReactionAppEvent::AssetSelection(asset_selection_event_command) => {
                if let Some(controller_id) = controller_id {
                    IrAssetSelectionEventSender::handle_event(
                        ir_app_event_sender_system_data,
                        controller_id,
                        entity,
                        asset_selection_event_command,
                    );
                } else {
                    error!("Expected `controller_id` to be set to send `AssetSelection` event.");
                }
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
            InputReactionAppEvent::SessionHost(session_host_event_command) => {
                IrSessionHostEventSender::handle_event(
                    ir_app_event_sender_system_data,
                    entity,
                    session_host_event_command,
                );
            }
            InputReactionAppEvent::SessionJoin(session_join_event_command) => {
                IrSessionJoinEventSender::handle_event(
                    ir_app_event_sender_system_data,
                    entity,
                    session_join_event_command,
                );
            }
            InputReactionAppEvent::SessionLobby(session_lobby_event_command) => {
                IrSessionLobbyEventSender::handle_event(
                    ir_app_event_sender_system_data,
                    session_lobby_event_command,
                );
            }
            InputReactionAppEvent::NetworkModeSelection(network_mode_selection_event_args) => {
                Self::handle_network_mode_selection_event(
                    ir_app_event_sender_system_data,
                    network_mode_selection_event_args,
                );
            }
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

    fn handle_network_mode_selection_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        network_mode_selection_event_args: NetworkModeSelectionEventArgs,
    ) {
        let network_mode_selection_event = match network_mode_selection_event_args {
            NetworkModeSelectionEventArgs::Select { index } => {
                NetworkModeSelectionEvent::Select(index)
            }
            NetworkModeSelectionEventArgs::Close => NetworkModeSelectionEvent::Close,
        };

        ir_app_event_sender_system_data
            .network_mode_selection_ec
            .single_write(network_mode_selection_event);
    }

    pub(crate) fn log_component_missing_error(
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
