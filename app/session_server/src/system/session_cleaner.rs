use std::net::SocketAddr;

use log::debug;
use network_session_model::play::SessionDeviceName;

use crate::{model::SocketToDeviceId, play::SessionTracker};

/// Functions to remove client information from session resources.
#[derive(Debug)]
pub struct SessionCleaner;

impl SessionCleaner {
    /// Removes session information for the given `SocketAddr` from session resources.
    pub fn client_forget(
        session_tracker: &mut SessionTracker<'_>,
        socket_to_device_id: &mut SocketToDeviceId,
        socket_addr: SocketAddr,
        session_device_name: &SessionDeviceName,
    ) {
        if let Some(session_code_existing) =
            session_tracker.remove_device_from_existing_session(socket_addr)
        {
            debug!(
                "Removed `{}` from existing session: `{}`.",
                session_device_name, session_code_existing
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
