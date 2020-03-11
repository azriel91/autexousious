use std::net::SocketAddr;

use amethyst::{
    derive::SystemDesc,
    ecs::{Read, ReadExpect, System, World, Write},
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
use net_model::play::NetMessageEvent;
use network_session_model::config::SessionServerConfig;

/// Sends requests to the session server.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetMessageRequestSystemDesc))]
pub struct NetMessageRequestSystem {
    /// Reader ID for the `NetMessageEvent` channel.
    #[system_desc(event_channel_reader)]
    net_message_event_rid: ReaderId<NetMessageEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetMessageRequestSystemData<'s> {
    /// `NetMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Read<'s, EventChannel<NetMessageEvent>>,
    /// `SessionServerConfig` resource.
    #[derivative(Debug = "ignore")]
    pub session_server_config: ReadExpect<'s, SessionServerConfig>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl<'s> System<'s> for NetMessageRequestSystem {
    type SystemData = NetMessageRequestSystemData<'s>;

    fn run(
        &mut self,
        NetMessageRequestSystemData {
            net_message_ec,
            session_server_config,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        let server_socket_addr =
            SocketAddr::new(session_server_config.address, session_server_config.port);
        net_message_ec
            .read(&mut self.net_message_event_rid)
            .for_each(|net_message_event| {
                match bincode::serialize(net_message_event) {
                    Ok(payload) => {
                        debug!("Sending `NetMessageEvent`: `{:?}`.", net_message_event);
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
                    }
                    Err(e) => error!("Failed to serialize `NetMessageEvent`. Error: `{}`.", e),
                }
            });
    }
}
