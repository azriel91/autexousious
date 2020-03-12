use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::NetworkSimulationEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use network_session_model::play::Sessions;

use crate::{model::SessionDeviceMappings, play::SessionTracker};

/// Listens for client disconnects, and removes them from the sessions.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionDeviceDisconnectResponderSystemDesc))]
pub struct SessionDeviceDisconnectResponderSystem {
    /// Reader ID for the `NetworkSimulationEvent` channel.
    #[system_desc(event_channel_reader)]
    network_simulation_event_rid: ReaderId<NetworkSimulationEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionDeviceDisconnectResponderSystemData<'s> {
    /// `NetworkSimulationEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_simulation_ec: Read<'s, EventChannel<NetworkSimulationEvent>>,
    /// `Sessions` resource.
    #[derivative(Debug = "ignore")]
    pub sessions: Write<'s, Sessions>,
    /// `SessionDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_device_mappings: Write<'s, SessionDeviceMappings>,
}

impl<'s> System<'s> for SessionDeviceDisconnectResponderSystem {
    type SystemData = SessionDeviceDisconnectResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionDeviceDisconnectResponderSystemData {
            network_simulation_ec,
            mut sessions,
            mut session_device_mappings,
        }: Self::SystemData,
    ) {
        let mut session_tracker = SessionTracker {
            sessions: &mut sessions,
            session_device_mappings: &mut session_device_mappings,
        };
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| {
                if let NetworkSimulationEvent::Disconnect(socket_addr) = ev {
                    if let Some(session_code) =
                        session_tracker.remove_device_from_existing_session(*socket_addr)
                    {
                        debug!(
                            "Device `{:?}` disconnected from session: `{}`.",
                            socket_addr, session_code
                        );
                        // TODO: broadcast disconnection event to remaining session devices.
                    }
                }
            });
    }
}
