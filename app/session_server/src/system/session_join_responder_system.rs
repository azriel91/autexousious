use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use net_model::play::NetMessage;
use network_session_model::play::{SessionDevice, SessionDeviceId, Sessions};
use session_join_model::{
    play::{SessionAcceptResponse, SessionJoinRequestParams, SessionRejectResponse},
    SessionJoinEvent,
};

/// Accepts or rejects session requests, and sends the response to the requester.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinResponderSystemDesc))]
pub struct SessionJoinResponderSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<SessionJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinResponderSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Read<'s, EventChannel<SessionJoinEvent>>,
    /// `Sessions` resource.
    #[derivative(Debug = "ignore")]
    pub sessions: Write<'s, Sessions>,
}

impl SessionJoinResponderSystem {
    fn handle_session_request(
        sessions: &mut Sessions,
        session_join_request_params: &SessionJoinRequestParams,
    ) -> SessionJoinEvent {
        let SessionJoinRequestParams {
            session_device_name,
            session_code,
        } = session_join_request_params;

        if let Some(session) = sessions.get_mut(session_code) {
            let session_device_id = session
                .session_devices
                .iter()
                .map(|session_device| session_device.id)
                .max()
                .map(|session_device_id| SessionDeviceId::new(*session_device_id + 1))
                .unwrap_or_else(|| SessionDeviceId::new(0));

            // Add the new device to the session before adding it to the response.
            session.session_devices.push(SessionDevice::new(
                session_device_id,
                session_device_name.clone(),
            ));

            debug!(
                "Session `{}` joined by `{}` with id: `{}`.",
                session_code, session_device_name, session_device_id
            );

            let session_accept_response =
                SessionAcceptResponse::new(session_device_id, session.clone());
            SessionJoinEvent::SessionAccept(session_accept_response)
        } else {
            debug!(
                "Rejecting request to join session `{}` joined from `{}`.",
                session_code, session_device_name
            );

            SessionJoinEvent::SessionReject(SessionRejectResponse::new(session_code.clone()))
        }
    }
}

impl<'s> System<'s> for SessionJoinResponderSystem {
    type SystemData = SessionJoinResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinResponderSystemData {
            session_join_ec,
            mut sessions,
        }: Self::SystemData,
    ) {
        let _net_messages = session_join_ec
            .read(&mut self.session_join_event_rid)
            .filter_map(|session_join_event| {
                if let SessionJoinEvent::SessionJoinRequest(session_join_request_params) =
                    session_join_event
                {
                    Some(session_join_request_params)
                } else {
                    None
                }
            })
            .map(|session_join_request_params| {
                Self::handle_session_request(&mut sessions, session_join_request_params)
            })
            .map(NetMessage::from);
    }
}
