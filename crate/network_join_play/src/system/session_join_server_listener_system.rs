use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::NetworkSimulationEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::{error, warn};
use network_join_model::NetworkJoinEvent;
use structopt::StructOpt;

/// Maps responses from session server into `NetworkJoinEvent`s.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinServerListenerSystemDesc))]
pub struct SessionJoinServerListenerSystem {
    /// Reader ID for the `NetworkSimulationEvent` channel.
    #[system_desc(event_channel_reader)]
    network_simulation_event_rid: ReaderId<NetworkSimulationEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinServerListenerSystemData<'s> {
    /// `NetworkSimulationEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_simulation_ec: Read<'s, EventChannel<NetworkSimulationEvent>>,
    /// `NetworkJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_join_ec: Write<'s, EventChannel<NetworkJoinEvent>>,
}

impl<'s> System<'s> for SessionJoinServerListenerSystem {
    type SystemData = SessionJoinServerListenerSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinServerListenerSystemData {
            mut network_join_ec,
            network_simulation_ec,
        }: Self::SystemData,
    ) {
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| match ev {
                NetworkSimulationEvent::Message(socket_addr, bytes) => {
                    warn!("Socket: {}, Message: {:?}", socket_addr, bytes);
                    let network_join_event = String::from_utf8(bytes.to_vec())
                        .map_err(|e| format!("{}", e))
                        .and_then(|args| shell_words::split(&args).map_err(|e| format!("{}", e)))
                        .and_then(|args| {
                            NetworkJoinEvent::from_iter_safe(args).map_err(|e| format!("{}", e))
                        });
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
