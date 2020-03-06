use std::net::SocketAddr;

use log::debug;
use net_model::play::{NetSessionDevice, NetSessionDevices};
use network_session_model::play::{
    Session, SessionCode, SessionDevice, SessionDeviceId, SessionDevices, Sessions,
};
use network_session_play::SessionCodeGenerator;
use session_host_model::play::SessionHostRequestParams;
use session_join_model::play::{SessionJoinError, SessionJoinRequestParams};

use crate::model::SessionDeviceMappings;

/// Updates tracking data for sessions.
#[derive(Debug)]
pub struct SessionTracker<'s> {
    /// Sessions (`HashMap<SessionCode, Session>` newtype).
    pub sessions: &'s mut Sessions,
    /// Mappings from `SessionCode` to `NetSessionDevices`, and `SocketAddr` to `SessionCode`.
    pub session_device_mappings: &'s mut SessionDeviceMappings,
}

impl<'s> SessionTracker<'s> {
    /// Registers and returns a new `Session` and the `SessionDeviceId` for the session host.
    ///
    /// # Parameters
    ///
    /// * `session_code_generator`: Generates session codes for sessions.
    /// * `socket_addr`: `SocketAddr` of the session host.
    /// * `session_host_request_params`: Parameters from the session hosting request.
    pub fn track_new(
        &mut self,
        session_code_generator: &mut SessionCodeGenerator,
        socket_addr: SocketAddr,
        session_host_request_params: &SessionHostRequestParams,
    ) -> (Session, SessionDeviceId) {
        let SessionHostRequestParams {
            session_device_name,
        } = session_host_request_params;

        let session_code = self.generate_session_code(session_code_generator);
        let session_device_id = SessionDeviceId::new(0); // ID for host
        let session_device = SessionDevice::new(session_device_id, session_device_name.clone());
        let session_devices = SessionDevices::new(vec![session_device.clone()]);

        let net_session_device = NetSessionDevice::new(socket_addr, session_device);
        let net_session_devices = NetSessionDevices::new(vec![net_session_device]);

        debug!(
            "Hosting new session `{}` by `{}` with id: `{}`.",
            session_code, session_device_name, session_device_id
        );

        let session = Session::new(session_code, session_devices);

        self.update_session_tracking(session.clone(), net_session_devices);

        (session, session_device_id)
    }

    pub fn append_device(
        &mut self,
        socket_addr: SocketAddr,
        session_join_request_params: &SessionJoinRequestParams,
    ) -> Result<(Session, SessionDevice), SessionJoinError> {
        let SessionJoinRequestParams {
            session_device_name,
            session_code,
        } = session_join_request_params;

        if let Some(session) = self.sessions.get_mut(session_code) {
            let session_device_id = session
                .session_devices
                .iter()
                .map(|session_device| session_device.id)
                .max()
                .map(|session_device_id| SessionDeviceId::new(*session_device_id + 1))
                .unwrap_or_else(|| SessionDeviceId::new(0));

            // Add the new device to the session before adding it to the response.
            let session_device = SessionDevice::new(session_device_id, session_device_name.clone());
            session.session_devices.push(session_device.clone());

            let net_session_device = NetSessionDevice::new(socket_addr, session_device.clone());
            self.session_device_mappings
                .append(session_code, net_session_device);

            debug!(
                "Session `{}` joined by `{}` with id: `{}`.",
                session_code, session_device.name, session_device.id
            );

            Ok((session.clone(), session_device))
        } else {
            Err(SessionJoinError::SessionCodeNotFound)
        }
    }

    fn generate_session_code(
        &mut self,
        session_code_generator: &mut SessionCodeGenerator,
    ) -> SessionCode {
        loop {
            let session_code = session_code_generator.generate();
            if !self.sessions.contains_key(&session_code) {
                break session_code;
            }
        }
    }

    fn update_session_tracking(
        &mut self,
        session: Session,
        net_session_devices: NetSessionDevices,
    ) {
        let session_code = session.session_code.clone();

        self.session_device_mappings
            .insert(&session_code, net_session_devices);

        self.sessions.insert(session_code, session);
    }
}
