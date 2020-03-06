use std::net::SocketAddr;

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
use net_model::play::{NetData, NetEventChannel, NetMessage};
use network_session_model::play::Sessions;
use session_join_model::{
    play::{SessionAcceptResponse, SessionJoinRequestParams, SessionRejectResponse},
    SessionJoinEvent,
};

use crate::{model::SessionDeviceMappings, play::SessionTracker};

/// Accepts or rejects session requests, and sends the response to the requester.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinResponderSystemDesc))]
pub struct SessionJoinResponderSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<NetData<SessionJoinEvent>>,
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
    /// `SessionDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_device_mappings: Write<'s, SessionDeviceMappings>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl SessionJoinResponderSystem {
    fn handle_session_request(
        session_tracker: &mut SessionTracker,
        socket_addr: SocketAddr,
        session_join_request_params: &SessionJoinRequestParams,
    ) -> SessionJoinEvent {
        let SessionJoinRequestParams {
            session_device_name,
            session_code,
        } = session_join_request_params;

        match session_tracker.append_device(socket_addr, session_join_request_params) {
            Ok((session, session_device_id)) => {
                let session_accept_response =
                    SessionAcceptResponse::new(session, session_device_id);

                SessionJoinEvent::SessionAccept(session_accept_response)
            }
            Err(e) => {
                debug!(
                    "Rejecting request to join session `{}` joined from `{}`.",
                    session_code, session_device_name
                );

                SessionJoinEvent::SessionReject(SessionRejectResponse::new(session_code.clone(), e))
            }
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
            mut session_device_mappings,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        let mut session_tracker = SessionTracker {
            sessions: &mut sessions,
            session_device_mappings: &mut session_device_mappings,
        };

        session_join_nec
            .read(&mut self.session_join_event_rid)
            .filter_map(|session_join_event| {
                if let NetData {
                    socket_addr,
                    data: SessionJoinEvent::SessionJoinRequest(session_join_request_params),
                } = session_join_event
                {
                    Some((*socket_addr, session_join_request_params))
                } else {
                    None
                }
            })
            .map(|(socket_addr, session_join_request_params)| {
                let session_join_event = Self::handle_session_request(
                    &mut session_tracker,
                    socket_addr,
                    session_join_request_params,
                );

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
