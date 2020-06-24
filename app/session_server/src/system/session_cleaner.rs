use std::net::SocketAddr;

use log::debug;
use net_model::play::NetSessionDevices;
use network_session_model::play::{SessionCode, SessionDeviceName, Sessions};

use crate::{
    model::{SessionDeviceMappings, SocketToDeviceId},
    play::SessionTracker,
};

/// Functions to remove client information from session resources.
#[derive(Debug)]
pub struct SessionCleaner;

impl SessionCleaner {
    /// Removes all devices and session information for the client with the given `SocketAddr`.
    ///
    /// This returns the session code and devices so that the caller may choose to send closing messages to the clients.
    pub fn session_forget(
        sessions: &mut Sessions,
        session_device_mappings: &mut SessionDeviceMappings<'_>,
        socket_to_device_id: &mut SocketToDeviceId,
        socket_addr: SocketAddr,
    ) -> Option<(SessionCode, NetSessionDevices)> {
        let session_code_and_devices = session_device_mappings.remove_session(&socket_addr);
        if let Some((session_code, net_session_devices)) = session_code_and_devices {
            {
                let device_names = net_session_devices
                    .iter()
                    .map(|net_session_device| &net_session_device.data.name)
                    .collect::<Vec<&SessionDeviceName>>();
                debug!(
                    "Removing session: `{}`, devices `{:?}`.",
                    session_code, device_names
                );
            }

            sessions.remove(&session_code);

            net_session_devices.iter().for_each(|net_session_device| {
                let device_id = socket_to_device_id.remove(&net_session_device.socket_addr);
                if let Some(device_id) = device_id {
                    debug!(
                        "Removed `{name}` [{id}] ({addr}) from `SocketToDeviceId` map.",
                        name = &net_session_device.data.name,
                        addr = socket_addr,
                        id = device_id
                    );
                }
            });

            Some((session_code, net_session_devices))
        } else {
            None
        }
    }

    /// Removes session information for the given `SocketAddr` from session resources.
    pub fn client_forget(
        session_tracker: &mut SessionTracker<'_>,
        socket_to_device_id: &mut SocketToDeviceId,
        socket_addr: SocketAddr,
        session_device_name: &SessionDeviceName,
    ) {
        if let Some((session_code, net_session_device)) =
            session_tracker.remove_device_from_existing_session(socket_addr)
        {
            debug!(
                "Removed `{}` from existing session: `{}`.",
                &net_session_device.data.name, session_code
            );
        }

        let device_id_previous = socket_to_device_id.remove(&socket_addr);
        if let Some(device_id_previous) = device_id_previous {
            debug!(
                "Removed `{name}` [{id}] ({addr}) from `SocketToDeviceId` map.",
                name = session_device_name,
                addr = socket_addr,
                id = device_id_previous
            );
        }
    }
}
