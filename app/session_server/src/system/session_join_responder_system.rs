use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shred::{ResourceId, SystemData},
    shrev::ReaderId,
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
use net_model::play::{NetEvent, NetEventChannel, NetMessage};
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
    session_join_event_rid: ReaderId<NetEvent<SessionJoinEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinResponderSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_nec: Read<'s, NetEventChannel<SessionJoinEvent>>,
    /// `Sessions` resource.
    #[derivative(Debug = "ignore")]
    pub sessions: Write<'s, Sessions>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
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
                SessionAcceptResponse::new(session.clone(), session_device_id);
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
            session_join_nec,
            mut sessions,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        session_join_nec
            .read(&mut self.session_join_event_rid)
            .filter_map(|session_join_event| {
                if let NetEvent {
                    socket_addr,
                    event: SessionJoinEvent::SessionJoinRequest(session_join_request_params),
                } = session_join_event
                {
                    Some((*socket_addr, session_join_request_params))
                } else {
                    None
                }
            })
            .map(|(socket_addr, session_join_request_params)| {
                let session_join_event =
                    Self::handle_session_request(&mut sessions, session_join_request_params);

                (socket_addr, NetMessage::from(session_join_event))
            })
            .for_each(|(socket_addr, net_message)| {
                match bincode::serialize(&net_message) {
                    Ok(payload) => {
                        transport_resource.send_with_requirements(
                            socket_addr,
                            &payload,
                            // None means it uses a default multiplexed stream.
                            //
                            // Suspect if we give it a value, the value will be a "channel" over the same
                            // socket connection.
                            DeliveryRequirement::ReliableOrdered(None),
                            UrgencyRequirement::OnTick,
                        );
                    }
                    Err(e) => {
                        error!(
                            "Failed to serialize `NetMessage::SessionJoinEvent`. Error: `{}`.",
                            e
                        );
                    }
                }
            });
    }
}
