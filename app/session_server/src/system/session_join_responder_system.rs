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
use net_model::play::{NetData, NetEventChannel, NetMessageEvent};
use network_session_model::{
    play::{SessionDeviceJoin, Sessions},
    SessionMessageEvent,
};
use session_join_model::{
    play::{SessionAcceptResponse, SessionJoinRequestParams, SessionRejectResponse},
    SessionJoinEvent,
};

use crate::{
    model::{SessionCodeToId, SessionDeviceMappings, SessionIdToDeviceMappings, SocketToDeviceId},
    play::SessionTracker,
    system::SessionCleaner,
};

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
    /// `SessionCodeToId` resource.
    #[derivative(Debug = "ignore")]
    pub session_code_to_id: Write<'s, SessionCodeToId>,
    /// `SessionIdToDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_id_to_device_mappings: Write<'s, SessionIdToDeviceMappings>,
    /// `SocketToDeviceId` resource.
    #[derivative(Debug = "ignore")]
    pub socket_to_device_id: Write<'s, SocketToDeviceId>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl SessionJoinResponderSystem {
    fn handle_session_request(
        session_tracker: &mut SessionTracker,
        socket_to_device_id: &mut SocketToDeviceId,
        socket_addr: SocketAddr,
        session_join_request_params: &SessionJoinRequestParams,
    ) -> (SessionJoinEvent, Option<SessionMessageEvent>) {
        let SessionJoinRequestParams {
            session_device_name,
            session_code,
            ..
        } = session_join_request_params;

        SessionCleaner::client_forget(
            session_tracker,
            socket_to_device_id,
            socket_addr,
            session_device_name,
        );

        match session_tracker.append_device(socket_addr, session_join_request_params) {
            Ok((session, session_device, player_controllers, controller_id_offset)) => {
                socket_to_device_id.insert(socket_addr, session_device.id);

                let session_accept_response = SessionAcceptResponse::new(
                    session,
                    session_device.id,
                    player_controllers.clone(),
                    controller_id_offset,
                );
                let session_join_event = SessionJoinEvent::SessionAccept(session_accept_response);

                let session_message_event = {
                    let session_device_join =
                        SessionDeviceJoin::new(session_device, player_controllers);
                    SessionMessageEvent::SessionDeviceJoin(session_device_join)
                };

                (session_join_event, Some(session_message_event))
            }
            Err(e) => {
                debug!(
                    "Rejecting request to join session `{}` joined from `{}`. Error: `{:?}`",
                    session_code, session_device_name, e
                );

                let session_join_event = SessionJoinEvent::SessionReject(
                    SessionRejectResponse::new(session_code.clone(), e),
                );

                (session_join_event, None)
            }
        }
    }

    fn send_session_join_event(
        transport_resource: &mut TransportResource,
        socket_addr: SocketAddr,
        session_join_event: SessionJoinEvent,
    ) {
        let net_message_event = NetMessageEvent::from(session_join_event);

        match bincode::serialize(&net_message_event) {
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
                    "Failed to serialize `NetMessageEvent::SessionJoinEvent`. Error: `{}`.",
                    e
                );
            }
        }
    }

    fn send_session_message_event(
        session_device_mappings: &SessionDeviceMappings,
        transport_resource: &mut TransportResource,
        socket_addr_exclude: SocketAddr,
        session_message_event: SessionMessageEvent,
    ) {
        let net_message_event = NetMessageEvent::from(session_message_event);

        match bincode::serialize(&net_message_event) {
            Ok(payload) => {
                let session_device_mappings_read = session_device_mappings.as_read();
                let net_session_devices = session_device_mappings_read
                    .session_code(&socket_addr_exclude)
                    .and_then(|session_code| {
                        session_device_mappings_read.net_session_devices(session_code)
                    });
                if let Some(net_session_devices) = net_session_devices {
                    net_session_devices
                        .iter()
                        .filter_map(|net_session_device| {
                            if net_session_device.socket_addr != socket_addr_exclude {
                                Some(net_session_device.socket_addr)
                            } else {
                                None
                            }
                        })
                        .for_each(|socket_addr| {
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
                        });
                }
            }
            Err(e) => {
                error!(
                    "Failed to serialize `NetMessageEvent::SessionJoinEvent`. Error: `{}`.",
                    e
                );
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
            mut session_code_to_id,
            mut session_id_to_device_mappings,
            mut socket_to_device_id,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        let session_code_to_id = &mut *session_code_to_id;
        let session_id_to_device_mappings = &mut *session_id_to_device_mappings;
        let mut session_device_mappings =
            SessionDeviceMappings::new(session_code_to_id, session_id_to_device_mappings);
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
                let session_join_and_message_events = Self::handle_session_request(
                    &mut session_tracker,
                    &mut socket_to_device_id,
                    socket_addr,
                    session_join_request_params,
                );

                (socket_addr, session_join_and_message_events)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(
                |(socket_addr, (session_join_event, session_message_event))| {
                    Self::send_session_join_event(
                        &mut transport_resource,
                        socket_addr,
                        session_join_event,
                    );

                    if let Some(session_message_event) = session_message_event {
                        Self::send_session_message_event(
                            &session_tracker.session_device_mappings,
                            &mut transport_resource,
                            socket_addr,
                            session_message_event,
                        );
                    }
                },
            );
    }
}
