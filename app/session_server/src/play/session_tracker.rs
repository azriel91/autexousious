use std::net::SocketAddr;

use game_input_model::{
    loaded::{PlayerController, PlayerControllers},
    play::ControllerIdOffset,
};
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
    ) -> (Session, SessionDeviceId, PlayerControllers) {
        let SessionHostRequestParams {
            session_device_name,
            player_controllers,
        } = session_host_request_params;

        let session_code = self.generate_session_code(session_code_generator);
        let session_device_id = SessionDeviceId::new(0); // ID for host
        let session_device = SessionDevice::new(
            session_device_id,
            session_device_name.clone(),
            player_controllers.clone(),
        );
        let session_devices = SessionDevices::new(vec![session_device.clone()]);

        let net_session_device = NetSessionDevice::new(socket_addr, session_device);
        let net_session_devices = NetSessionDevices::new(vec![net_session_device]);

        debug!(
            "Hosting new session `{}` by `{}` with id: `{}`.",
            session_code, session_device_name, session_device_id
        );

        let session = Session::new(session_code, session_devices);

        self.update_session_tracking(session.clone(), net_session_devices);

        (session, session_device_id, player_controllers.clone())
    }

    pub fn append_device(
        &mut self,
        socket_addr: SocketAddr,
        session_join_request_params: &SessionJoinRequestParams,
    ) -> Result<
        (
            Session,
            SessionDevice,
            PlayerControllers,
            ControllerIdOffset,
        ),
        SessionJoinError,
    > {
        let SessionJoinRequestParams {
            session_code,
            session_device_name,
            player_controllers,
        } = session_join_request_params;

        if let Some(session) = self.sessions.get_mut(session_code) {
            let session_device_id = session
                .session_devices
                .iter()
                .map(|session_device| session_device.id)
                .max()
                .map(|session_device_id| SessionDeviceId::new(*session_device_id + 1))
                .unwrap_or_else(|| SessionDeviceId::new(0));

            // Mutate the `ControllerId`s on the `player_controllers`.
            let controller_id_offset: ControllerIdOffset = ControllerIdOffset::new(
                session
                    .session_devices
                    .iter()
                    .map(|session_device| session_device.player_controllers.len())
                    .sum(),
            );
            let mut player_controllers = player_controllers.clone();
            player_controllers.iter_mut().for_each(|player_controller| {
                player_controller.controller_id += controller_id_offset.0
            });

            // Add the new device to the session before adding it to the response.
            let session_device = SessionDevice::new(
                session_device_id,
                session_device_name.clone(),
                player_controllers,
            );
            session.session_devices.push(session_device.clone());

            let net_session_device = NetSessionDevice::new(socket_addr, session_device.clone());
            self.session_device_mappings
                .append(session_code, net_session_device);

            debug!(
                "Session `{}` joined by `{}` with id: `{}`.",
                session_code, session_device.name, session_device.id
            );

            // Compute combined player controllers
            let player_controllers_all = session
                .session_devices
                .iter()
                .flat_map(|session_device| session_device.player_controllers.iter().cloned())
                .collect::<Vec<PlayerController>>();
            let player_controllers_all = PlayerControllers::new(player_controllers_all);

            Ok((
                session.clone(),
                session_device,
                player_controllers_all,
                controller_id_offset,
            ))
        } else {
            Err(SessionJoinError::SessionCodeNotFound)
        }
    }

    /// Removes the device from any previous session, returning the session code.
    ///
    /// # Parameters
    ///
    /// * `socket_addr`: `SocketAddr` of the session device.
    pub fn remove_device_from_existing_session(
        &mut self,
        socket_addr: SocketAddr,
    ) -> Option<&SessionCode> {
        self.session_device_mappings.remove_device(&socket_addr)
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
