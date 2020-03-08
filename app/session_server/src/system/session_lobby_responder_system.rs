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
use session_lobby_model::{play::SessionStartRequestParams, SessionLobbyEvent};

use crate::model::SessionDeviceMappings;

/// Accepts or rejects session start requests, and notifies all connected devices.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionLobbyResponderSystemDesc))]
pub struct SessionLobbyResponderSystem {
    /// Reader ID for the `SessionLobbyEvent` channel.
    #[system_desc(event_channel_reader)]
    session_lobby_event_rid: ReaderId<NetData<SessionLobbyEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionLobbyResponderSystemData<'s> {
    /// `SessionLobbyEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_lobby_nec: Read<'s, NetEventChannel<SessionLobbyEvent>>,
    /// `SessionDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_device_mappings: Read<'s, SessionDeviceMappings>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl SessionLobbyResponderSystem {
    fn send_session_lobby_event(
        transport_resource: &mut TransportResource,
        socket_addrs: impl Iterator<Item = SocketAddr>,
        session_lobby_event: SessionLobbyEvent,
    ) {
        let net_message = NetMessage::from(session_lobby_event);

        match bincode::serialize(&net_message) {
            Ok(payload) => {
                socket_addrs.for_each(|socket_addr| {
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
            Err(e) => {
                error!(
                    "Failed to serialize `NetMessage::SessionLobbyEvent`. Error: `{}`.",
                    e
                );
            }
        }
    }
}

impl<'s> System<'s> for SessionLobbyResponderSystem {
    type SystemData = SessionLobbyResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionLobbyResponderSystemData {
            session_lobby_nec,
            session_device_mappings,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        session_lobby_nec
            .read(&mut self.session_lobby_event_rid)
            .filter_map(|session_lobby_event| {
                if let NetData {
                    socket_addr,
                    data: SessionLobbyEvent::SessionStartRequest(session_start_request_params),
                } = session_lobby_event
                {
                    Some((*socket_addr, session_start_request_params))
                } else {
                    None
                }
            })
            .for_each(|(socket_addr, session_start_request_params)| {
                let SessionStartRequestParams { session_code } = session_start_request_params;

                // Make sure the start request is for the correct `session_code`.
                if let Some(session_code_tracked) =
                    session_device_mappings.session_code(&socket_addr)
                {
                    if session_code_tracked == session_code {
                        if let Some(net_session_devices) =
                            session_device_mappings.net_session_devices(session_code)
                        {
                            debug!(
                                "Sending `SessionStartNotify` for session: `{}`.",
                                session_code
                            );

                            let socket_addrs = net_session_devices
                                .iter()
                                .map(|net_session_device| net_session_device.socket_addr);
                            Self::send_session_lobby_event(
                                &mut transport_resource,
                                socket_addrs,
                                SessionLobbyEvent::SessionStartNotify,
                            );
                        }
                    } else {
                        debug!(
                            "Received `{:?}` from {:?}, but session code tracked is `{}`.",
                            session_start_request_params, socket_addr, session_code_tracked,
                        );
                        // TODO: reject because the session code doesn't match
                    }
                } else {
                    debug!(
                        "Received `{:?}` from {:?}, but no session code tracked for that socket.",
                        session_start_request_params, socket_addr
                    );
                    // TODO: reject
                }
            });
    }
}
