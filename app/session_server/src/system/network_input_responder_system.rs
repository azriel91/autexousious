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
use game_input_model::GameInputEvent;
use log::{debug, error};
use net_model::play::{NetData, NetEventChannel, NetMessageEvent};

use crate::model::SessionDeviceMappings;

/// Broadcasts `InputEvent`s to connected devices within the same session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetworkInputResponderSystemDesc))]
pub struct NetworkInputResponderSystem {
    /// Reader ID for the `GameInputEvent` channel.
    #[system_desc(event_channel_reader)]
    game_input_event_rid: ReaderId<NetData<GameInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetworkInputResponderSystemData<'s> {
    /// `InputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_input_nec: Read<'s, NetEventChannel<GameInputEvent>>,
    /// `SessionDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_device_mappings: Read<'s, SessionDeviceMappings>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl NetworkInputResponderSystem {
    fn send_game_input_event(
        transport_resource: &mut TransportResource,
        socket_addrs: impl Iterator<Item = SocketAddr>,
        game_input_event: GameInputEvent,
    ) {
        let net_message_event = NetMessageEvent::from(game_input_event);

        match bincode::serialize(&net_message_event) {
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
                    "Failed to serialize `NetMessageEvent::InputEvent`. Error: `{}`.",
                    e
                );
            }
        }
    }
}

impl<'s> System<'s> for NetworkInputResponderSystem {
    type SystemData = NetworkInputResponderSystemData<'s>;

    fn run(
        &mut self,
        NetworkInputResponderSystemData {
            network_input_nec,
            session_device_mappings,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        network_input_nec
            .read(&mut self.game_input_event_rid)
            .for_each(|net_game_input_event| {
                let NetData {
                    socket_addr,
                    data: game_input_event,
                } = net_game_input_event;

                if let Some(session_code) = session_device_mappings.session_code(&socket_addr) {
                    if let Some(net_session_devices) =
                        session_device_mappings.net_session_devices(session_code)
                    {
                        debug!("Sending `GameInputEvent` for session: `{}`.", session_code);

                        let socket_addrs = net_session_devices
                            .iter()
                            .map(|net_session_device| net_session_device.socket_addr);
                        Self::send_game_input_event(
                            &mut transport_resource,
                            socket_addrs,
                            game_input_event.clone(),
                        );
                    }
                } else {
                    debug!(
                        "Received `{:?}` from {:?}, but no session code tracked for that socket.",
                        game_input_event, socket_addr
                    );
                    // TODO: reject
                }
            });
    }
}
