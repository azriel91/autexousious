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
use network_session_model::play::SessionStatus;
use session_join_model::{config::SessionServerConfig, SessionJoinEvent};

/// Builds a `SessionJoinRequestSystem`.
#[derive(Debug, Default, new)]
pub struct SessionJoinRequestSystemDesc {
    /// Configuration needed to connect to the session server.
    pub session_server_config: SessionServerConfig,
}

impl<'a, 'b> SystemDesc<'a, 'b, SessionJoinRequestSystem> for SessionJoinRequestSystemDesc {
    fn build(self, world: &mut World) -> SessionJoinRequestSystem {
        <SessionJoinRequestSystem as System<'_>>::SystemData::setup(world);

        let session_join_event_rid = world
            .fetch_mut::<EventChannel<SessionJoinEvent>>()
            .register_reader();

        world.insert(self.session_server_config);

        SessionJoinRequestSystem::new(session_join_event_rid)
    }
}

/// Sends requests to a game server to join a session.
#[derive(Debug, new)]
pub struct SessionJoinRequestSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    session_join_event_rid: ReaderId<SessionJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinRequestSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Read<'s, EventChannel<SessionJoinEvent>>,
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

impl<'s> System<'s> for SessionJoinRequestSystem {
    type SystemData = SessionJoinRequestSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinRequestSystemData {
            session_join_ec,
            mut session_status,
            session_server_config,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        let mut session_join_events = session_join_ec.read(&mut self.session_join_event_rid);

        // Guard against requesting multiple sessions at the same time.
        if *session_status != SessionStatus::None {
            return;
        }

        // Only process one session join event if multiple are received.
        let session_join_request_params = session_join_events.find_map(|ev| {
            if let SessionJoinEvent::SessionJoinRequest(session_join_request_params) = ev {
                Some(session_join_request_params)
            } else {
                None
            }
        });

        if let Some(session_join_request_params) = session_join_request_params {
            let server_socket_addr =
                SocketAddr::new(session_server_config.address, session_server_config.port);

            match bincode::serialize(&NetMessage::SessionJoinEvent(
                SessionJoinEvent::SessionJoinRequest(session_join_request_params.clone()),
            )) {
                Ok(payload) => {
                    // Connect to `server_socket_addr` and send request.
                    transport_resource.send_with_requirements(
                        server_socket_addr,
                        &payload,
                        // None means it uses a default multiplexed stream.
                        //
                        // Suspect if we give it a value, the value will be a "channel" over the same
                        // socket connection.
                        DeliveryRequirement::ReliableOrdered(None),
                        UrgencyRequirement::OnTick,
                    );
                    *session_status = SessionStatus::JoinRequested {
                        session_code: session_join_request_params.session_code.clone(),
                    };
                }
                Err(e) => {
                    error!(
                        "Failed to serialize `NetMessage::SessionJoinEvent`. Error: `{}`.",
                        e
                    );
                }
            }
        }
    }
}
