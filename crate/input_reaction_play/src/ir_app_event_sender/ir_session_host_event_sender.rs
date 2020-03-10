use amethyst::ecs::Entity;
use network_session_model::play::SessionDeviceName;
use session_host_model::{
    config::SessionHostEventCommand, play::SessionHostRequestParams, SessionHostEvent,
};

use crate::IrAppEventSenderSystemData;

/// Handles sending `SessionHostEvent`s from input reactions.
#[derive(Debug)]
pub struct IrSessionHostEventSender;

impl IrSessionHostEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        _entity: Entity,
        host_event_command: SessionHostEventCommand,
    ) {
        let host_event = match host_event_command {
            SessionHostEventCommand::SessionHostRequest => {
                if let Some(host_request_params) =
                    Self::host_request_params_discover(ir_app_event_sender_system_data)
                {
                    Some(SessionHostEvent::SessionHostRequest(host_request_params))
                } else {
                    // TODO: Feedback that the form needs to be filled.
                    None
                }
            }
            SessionHostEventCommand::HostCancel => Some(SessionHostEvent::HostCancel),
            SessionHostEventCommand::Back => Some(SessionHostEvent::Back),
        };

        if let Some(host_event) = host_event {
            ir_app_event_sender_system_data
                .session_host_ec
                .single_write(host_event);
        }
    }

    fn host_request_params_discover(
        ir_app_event_sender_system_data: &IrAppEventSenderSystemData,
    ) -> Option<SessionHostRequestParams> {
        let IrAppEventSenderSystemData {
            player_controllers,
            ui_form_input_entities,
            ui_texts,
            ..
        } = ir_app_event_sender_system_data;

        let mut ui_form_input_iter = ui_form_input_entities.iter().copied();
        let session_device_name = ui_form_input_iter
            .next()
            .and_then(|entity| ui_texts.get(entity))
            .map(|ui_text| ui_text.text.clone())
            .map(SessionDeviceName::new);

        let player_controllers = (*player_controllers).clone();

        if let Some(session_device_name) = session_device_name {
            Some(SessionHostRequestParams::new(
                session_device_name,
                player_controllers,
            ))
        } else {
            None
        }
    }
}
