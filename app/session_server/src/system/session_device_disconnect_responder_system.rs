use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::NetworkSimulationEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::play::Sessions;

use crate::{
    model::{SessionCodeToId, SessionDeviceMappings, SessionIdToDeviceMappings, SocketToDeviceId},
    system::SessionCleaner,
};

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
    /// `SessionCodeToId` resource.
    #[derivative(Debug = "ignore")]
    pub session_code_to_id: Write<'s, SessionCodeToId>,
    /// `SocketToDeviceId` resource.
    #[derivative(Debug = "ignore")]
    pub socket_to_device_id: Write<'s, SocketToDeviceId>,
    /// `SessionIdToDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_id_to_device_mappings: Write<'s, SessionIdToDeviceMappings>,
}

impl<'s> System<'s> for SessionDeviceDisconnectResponderSystem {
    type SystemData = SessionDeviceDisconnectResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionDeviceDisconnectResponderSystemData {
            network_simulation_ec,
            mut sessions,
            mut session_code_to_id,
            mut socket_to_device_id,
            mut session_id_to_device_mappings,
        }: Self::SystemData,
    ) {
        let session_code_to_id = &mut *session_code_to_id;
        let session_id_to_device_mappings = &mut *session_id_to_device_mappings;
        let mut session_device_mappings =
            SessionDeviceMappings::new(session_code_to_id, session_id_to_device_mappings);
        network_simulation_ec
            .read(&mut self.network_simulation_event_rid)
            .for_each(|ev| {
                if let NetworkSimulationEvent::Disconnect(socket_addr) = ev {
                    // Forget all clients in the session.
                    let session_code_and_devices = SessionCleaner::session_forget(
                        &mut sessions,
                        &mut session_device_mappings,
                        &mut socket_to_device_id,
                        *socket_addr,
                    );

                    if let Some((_session_code, _net_session_devices)) = session_code_and_devices {
                        // TODO: Send disconnect message to all clients except the one that disconnected.
                    }
                }
            });
    }
}
