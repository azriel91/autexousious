use amethyst::ecs::Entity;
use network_join_model::{
    config::NetworkJoinEventCommand, play::SessionJoinRequestParams, NetworkJoinEvent,
};

use crate::IrAppEventSenderSystemData;

/// Handles sending `NetworkJoinEvent`s from input reactions.
#[derive(Debug)]
pub struct IrNetworkJoinEventSender;

impl IrNetworkJoinEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        _entity: Entity,
        network_join_event_command: NetworkJoinEventCommand,
    ) {
        let network_join_event = match network_join_event_command {
            NetworkJoinEventCommand::SessionJoinRequest => {
                if let Some(session_join_request_params) =
                    Self::session_join_request_params_discover()
                {
                    Some(NetworkJoinEvent::SessionJoinRequest(
                        session_join_request_params,
                    ))
                } else {
                    // TODO: Feedback that the form needs to be filled.
                    None
                }
            }
            NetworkJoinEventCommand::JoinCancel => Some(NetworkJoinEvent::JoinCancel),
            NetworkJoinEventCommand::Back => Some(NetworkJoinEvent::Back),
        };

        if let Some(network_join_event) = network_join_event {
            ir_app_event_sender_system_data
                .network_join_ec
                .single_write(network_join_event);
        }
    }

    fn session_join_request_params_discover() -> Option<SessionJoinRequestParams> {
        let session_device_name = None;
        let session_code = None;

        if let (Some(session_device_name), Some(session_code)) = (session_device_name, session_code)
        {
            Some(SessionJoinRequestParams::new(
                session_device_name,
                session_code,
            ))
        } else {
            None
        }
    }
}
