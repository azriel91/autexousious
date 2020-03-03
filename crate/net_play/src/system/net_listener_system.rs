use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::NetworkSimulationEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
use net_model::play::{NetEvent, NetEventChannel, NetMessage};
use session_host_model::SessionHostEvent;
use session_join_model::SessionJoinEvent;

/// Receives `NetMessage`s and sends each variant's data to the corresponding event channel.
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
    /// Net `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_nec: Write<'s, NetEventChannel<SessionHostEvent>>,
    /// Net `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_nec: Write<'s, NetEventChannel<SessionJoinEvent>>,
}

impl<'s> System<'s> for NetListenerSystem {
    type SystemData = NetListenerSystemData<'s>;

    fn run(
        &mut self,
        NetListenerSystemData {
            network_simulation_ec,
            mut session_host_nec,
            mut session_join_nec,
        }: Self::SystemData,
    ) {
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| match ev {
                NetworkSimulationEvent::Message(socket_addr, bytes) => {
                    debug!("Socket: {}, Message: {:?}", socket_addr, bytes);
                    let net_message = bincode::deserialize(bytes);
                    match net_message {
                        Ok(net_message) => {
                            debug!("{:?}", net_message);
                            match net_message {
                                NetMessage::SessionHostEvent(session_host_event) => {
                                    session_host_nec.single_write(NetEvent::new(
                                        *socket_addr,
                                        session_host_event,
                                    ));
                                }
                                NetMessage::SessionJoinEvent(session_join_event) => {
                                    session_join_nec.single_write(NetEvent::new(
                                        *socket_addr,
                                        session_join_event,
                                    ));
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse `NetMessage`: `{}`", e),
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
