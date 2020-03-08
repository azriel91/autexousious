use session_lobby_model::{
    config::SessionLobbyEventCommand, play::SessionStartRequestParams, SessionLobbyEvent,
};

use crate::IrAppEventSenderSystemData;

/// Handles sending `SessionLobbyEvent`s from input reactions.
#[derive(Debug)]
pub struct IrSessionLobbyEventSender;

impl IrSessionLobbyEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        lobby_event_command: SessionLobbyEventCommand,
    ) {
        let lobby_event = match lobby_event_command {
            SessionLobbyEventCommand::SessionStartRequest => {
                let session_code = (*ir_app_event_sender_system_data.session_code).clone();
                let session_start_request_params = SessionStartRequestParams::new(session_code);
                SessionLobbyEvent::SessionStartRequest(session_start_request_params)
            }
            SessionLobbyEventCommand::Back => SessionLobbyEvent::Back,
        };

        ir_app_event_sender_system_data
            .session_lobby_ec
            .single_write(lobby_event);
    }
}
