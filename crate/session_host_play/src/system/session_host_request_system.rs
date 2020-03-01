use std::net::SocketAddr;

use amethyst::{
    core::SystemDesc,
    ecs::{Read, ReadExpect, System, World, Write},
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::error;
use net_model::play::NetMessage;
use network_session_model::{config::SessionServerConfig, play::SessionStatus};
use session_host_model::SessionHostEvent;

/// Builds a `SessionHostRequestSystem`.
#[derive(Debug, Default, new)]
pub struct SessionHostRequestSystemDesc {
    /// Configuration needed to connect to the session server.
    pub session_server_config: SessionServerConfig,
}

impl<'a, 'b> SystemDesc<'a, 'b, SessionHostRequestSystem> for SessionHostRequestSystemDesc {
    fn build(self, world: &mut World) -> SessionHostRequestSystem {
        <SessionHostRequestSystem as System<'_>>::SystemData::setup(world);

        let session_host_event_rid = world
            .fetch_mut::<EventChannel<SessionHostEvent>>()
            .register_reader();

        world.insert(self.session_server_config);

        SessionHostRequestSystem::new(session_host_event_rid)
    }
}

/// Sends requests to a game server to host a session.
#[derive(Debug, new)]
pub struct SessionHostRequestSystem {
    /// Reader ID for the `SessionHostEvent` channel.
    session_host_event_rid: ReaderId<SessionHostEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionHostRequestSystemData<'s> {
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_ec: Read<'s, EventChannel<SessionHostEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Write<'s, SessionStatus>,
    /// `SessionServerConfig` resource.
    #[derivative(Debug = "ignore")]
    pub session_server_config: ReadExpect<'s, SessionServerConfig>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl<'s> System<'s> for SessionHostRequestSystem {
    type SystemData = SessionHostRequestSystemData<'s>;

    fn run(
        &mut self,
        SessionHostRequestSystemData {
            session_host_ec,
            mut session_status,
            session_server_config,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        let mut session_host_events = session_host_ec.read(&mut self.session_host_event_rid);

        // Guard against requesting multiple sessions at the same time.
        if *session_status != SessionStatus::None {
            return;
        }

        // Only process one session host event if multiple are received.
        let session_host_request_params = session_host_events.find_map(|ev| {
            if let SessionHostEvent::SessionHostRequest(session_host_request_params) = ev {
                Some(session_host_request_params)
            } else {
                None
            }
        });

        if let Some(session_host_request_params) = session_host_request_params {
            let server_socket_addr =
                SocketAddr::new(session_server_config.address, session_server_config.port);

            match bincode::serialize(&NetMessage::SessionHostEvent(
                SessionHostEvent::SessionHostRequest(session_host_request_params.clone()),
            )) {
                Ok(payload) => {
                    // Connect to `server_socket_addr` and send request.
                    transport_resource.send_with_requirements(
                        server_socket_addr,
                        &payload,
                        // None means it uses a default multiplexed stream.
                        //
                        // Suspect if we give it a value, the value will be a "channel" over the
                        // same socket connection.
                        DeliveryRequirement::ReliableOrdered(None),
                        UrgencyRequirement::OnTick,
                    );
                    *session_status = SessionStatus::HostRequested;
                }
                Err(e) => {
                    error!(
                        "Failed to serialize `NetMessage::SessionHostEvent`. Error: `{}`.",
                        e
                    );
                }
            }
        }
    }
}
