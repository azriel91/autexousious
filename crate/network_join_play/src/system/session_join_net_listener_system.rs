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
use network_join_model::NetworkJoinEvent;

/// Maps requests from session clients into `NetworkJoinEvent`s.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinNetListenerSystemDesc))]
pub struct SessionJoinNetListenerSystem {
    /// Reader ID for the `NetworkSimulationEvent` channel.
    #[system_desc(event_channel_reader)]
    network_simulation_event_rid: ReaderId<NetworkSimulationEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinNetListenerSystemData<'s> {
    /// `NetworkSimulationEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_simulation_ec: Read<'s, EventChannel<NetworkSimulationEvent>>,
    /// `NetworkJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_join_ec: Write<'s, EventChannel<NetworkJoinEvent>>,
}

impl<'s> System<'s> for SessionJoinNetListenerSystem {
    type SystemData = SessionJoinNetListenerSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinNetListenerSystemData {
            mut network_join_ec,
            network_simulation_ec,
        }: Self::SystemData,
    ) {
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| match ev {
                NetworkSimulationEvent::Message(socket_addr, bytes) => {
                    debug!("Socket: {}, Message: {:?}", socket_addr, bytes);
                    let network_join_event = bincode::deserialize(bytes);
                    match network_join_event {
                        Ok(network_join_event) => {
                            debug!("{:?}", network_join_event);
                            network_join_ec.single_write(network_join_event)
                        }
                        Err(e) => error!("Failed to parse `NetworkJoinEvent`: `{}`", e),
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
