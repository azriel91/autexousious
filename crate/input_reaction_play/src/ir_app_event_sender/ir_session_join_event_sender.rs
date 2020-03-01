use amethyst::ecs::Entity;
use network_session_model::play::{SessionCode, SessionDeviceName};
use session_join_model::{
    config::SessionJoinEventCommand, play::SessionJoinRequestParams, SessionJoinEvent,
};

use crate::IrAppEventSenderSystemData;

/// Handles sending `SessionJoinEvent`s from input reactions.
#[derive(Debug)]
pub struct IrSessionJoinEventSender;

impl IrSessionJoinEventSender {
    pub fn handle_event(
        ir_app_event_sender_system_data: &mut IrAppEventSenderSystemData,
        _entity: Entity,
        session_join_event_command: SessionJoinEventCommand,
    ) {
        let session_join_event = match session_join_event_command {
            SessionJoinEventCommand::SessionJoinRequest => {
                if let Some(session_join_request_params) =
                    Self::session_join_request_params_discover(ir_app_event_sender_system_data)
                {
                    Some(SessionJoinEvent::SessionJoinRequest(
                        session_join_request_params,
                    ))
                } else {
                    // TODO: Feedback that the form needs to be filled.
                    None
                }
            }
            SessionJoinEventCommand::JoinCancel => Some(SessionJoinEvent::JoinCancel),
            SessionJoinEventCommand::Back => Some(SessionJoinEvent::Back),
        };

        if let Some(session_join_event) = session_join_event {
            ir_app_event_sender_system_data
                .session_join_ec
                .single_write(session_join_event);
        }
    }

    fn session_join_request_params_discover(
        ir_app_event_sender_system_data: &IrAppEventSenderSystemData,
    ) -> Option<SessionJoinRequestParams> {
        let IrAppEventSenderSystemData {
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
        let session_code = ui_form_input_iter
            .next()
            .and_then(|entity| ui_texts.get(entity))
            .map(|ui_text| ui_text.text.clone())
            .map(SessionCode::new);

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
