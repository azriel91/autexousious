use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    input::InputEvent,
    network::simulation::NetworkSimulationEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::config::ControlBindings;
use log::{debug, error};
use net_model::play::{NetData, NetEventChannel, NetMessageEvent};
use network_session_model::SessionMessageEvent;
use session_host_model::SessionHostEvent;
use session_join_model::SessionJoinEvent;
use session_lobby_model::SessionLobbyEvent;

/// Receives `NetMessageEvent`s and sends each variant's data to the corresponding event channel.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetListenerSystemDesc))]
pub struct NetListenerSystem {
    /// Reader ID for the `NetworkSimulationEvent` channel.
    #[system_desc(event_channel_reader)]
    network_simulation_event_rid: ReaderId<NetworkSimulationEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetListenerSystemData<'s> {
    /// `NetworkSimulationEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_simulation_ec: Read<'s, EventChannel<NetworkSimulationEvent>>,
    /// Net `InputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub input_nec: Write<'s, NetEventChannel<InputEvent<ControlBindings>>>,
    /// Net `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_nec: Write<'s, NetEventChannel<SessionHostEvent>>,
    /// Net `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_nec: Write<'s, NetEventChannel<SessionJoinEvent>>,
    /// Net `SessionLobbyEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_lobby_nec: Write<'s, NetEventChannel<SessionLobbyEvent>>,
    /// Net `SessionMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_message_nec: Write<'s, NetEventChannel<SessionMessageEvent>>,
}

impl<'s> System<'s> for NetListenerSystem {
    type SystemData = NetListenerSystemData<'s>;

    fn run(
        &mut self,
        NetListenerSystemData {
            network_simulation_ec,
            mut input_nec,
            mut session_host_nec,
            mut session_join_nec,
            mut session_lobby_nec,
            mut session_message_nec,
        }: Self::SystemData,
    ) {
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| match ev {
                NetworkSimulationEvent::Message(socket_addr, bytes) => {
                    debug!("Socket: {}, Message: {:?}", socket_addr, bytes);
                    let net_message_event = bincode::deserialize(bytes);
                    match net_message_event {
                        Ok(net_message_event) => {
                            debug!("{:?}", net_message_event);
                            match net_message_event {
                                NetMessageEvent::InputEvent(input_event) => {
                                    input_nec.single_write(NetData::new(*socket_addr, input_event));
                                }
                                NetMessageEvent::SessionHostEvent(session_host_event) => {
                                    session_host_nec.single_write(NetData::new(
                                        *socket_addr,
                                        session_host_event,
                                    ));
                                }
                                NetMessageEvent::SessionJoinEvent(session_join_event) => {
                                    session_join_nec.single_write(NetData::new(
                                        *socket_addr,
                                        session_join_event,
                                    ));
                                }
                                NetMessageEvent::SessionLobbyEvent(session_lobby_event) => {
                                    session_lobby_nec.single_write(NetData::new(
                                        *socket_addr,
                                        session_lobby_event,
                                    ));
                                }
                                NetMessageEvent::SessionMessageEvent(session_message_event) => {
                                    session_message_nec.single_write(NetData::new(
                                        *socket_addr,
                                        session_message_event,
                                    ));
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse `NetMessageEvent`: `{}`", e),
                    }
                }
                NetworkSimulationEvent::SendError(io_error, message) => {
                    error!("Send error: `{}`, message: `{:?}`", io_error, message);
                }
                NetworkSimulationEvent::RecvError(io_error) => {
                    error!("Receive error: `{}`", io_error);
                }
                NetworkSimulationEvent::ConnectionError(io_error, socket_addr) => {
                    error!(
                        "Connection error: `{}`, socket_addr: `{:?}`",
                        io_error, socket_addr
                    );
                }
                _ => {}
            });
    }
}
