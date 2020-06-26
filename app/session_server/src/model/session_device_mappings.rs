use std::net::SocketAddr;

use derive_new::new;
use net_model::play::{NetSessionDevice, NetSessionDevices};
use network_session_model::play::SessionCode;

use crate::model::{SessionCodeToId, SessionIdToDeviceMappings};

/// Abstraction layer for mappings between `SessionCode` and `NetSessionDevice`s.
///
/// This hides the detail of mapping between `SessionCode` and `SessionCodeId` when managing the `NetSessionDevices` for
/// a session.
#[derive(Clone, Copy, Debug, new)]
pub struct SessionDeviceMappingsRead<'sdm> {
    /// Bidirectional mappings from `SessionCode` to `SessionCodeId`.
    pub session_code_to_id: &'sdm SessionCodeToId,
    /// Mappings from `SessionCodeId` to `NetSessionDevices`, and `SocketAddr` to `SessionCodeId`.
    pub session_id_to_device_mappings: &'sdm SessionIdToDeviceMappings,
}

/// Abstraction layer for mappings between `SessionCode` and `NetSessionDevice`s.
///
/// This hides the detail of mapping between `SessionCode` and `SessionCodeId` when managing the `NetSessionDevices` for
/// a session.
#[derive(Debug, new)]
pub struct SessionDeviceMappings<'sdm> {
    /// Bidirectional mappings from `SessionCode` to `SessionCodeId`.
    session_code_to_id: &'sdm mut SessionCodeToId,
    /// Mappings from `SessionCodeId` to `NetSessionDevices`, and `SocketAddr` to `SessionCodeId`.
    session_id_to_device_mappings: &'sdm mut SessionIdToDeviceMappings,
}

impl<'sdm> SessionDeviceMappingsRead<'sdm> {
    /// Returns the `NetSessionDevices` for the given `SessionCode`.
    pub fn net_session_devices(&self, session_code: &SessionCode) -> Option<&NetSessionDevices> {
        self.session_code_to_id
            .get_by_left(session_code)
            .and_then(|session_code_id| {
                self.session_id_to_device_mappings
                    .net_session_devices(*session_code_id)
            })
    }

    /// Returns the `SessionCode` for the given `SocketAddr`.
    pub fn session_code(&self, socket_addr: &SocketAddr) -> Option<&SessionCode> {
        self.session_id_to_device_mappings
            .session_code_id(socket_addr)
            .as_ref()
            .and_then(|session_code_id| self.session_code_to_id.get_by_right(session_code_id))
    }

    /// Returns `true` if there are no sessions.
    pub fn is_empty(&self) -> bool {
        self.session_code_to_id.is_empty()
    }

    /// Returns an iterator of `SessionCode`s to `NetSessionDevices`.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&SessionCode, &NetSessionDevices)> + 'a {
        self.session_code_to_id
            .iter()
            .map(move |(session_code, session_code_id)| {
                let net_session_devices = self
                    .session_id_to_device_mappings
                    .net_session_devices(*session_code_id)
                    .expect("Expected `NetSessionDevices` to exist.");
                (session_code, net_session_devices)
            })
    }

    /// Returns an iterator visiting all `SessionCode`s in arbitrary order.
    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &SessionCode> + 'a {
        self.session_code_to_id.left_values()
    }

    /// Returns an iterator visiting all `NetSessionDevices` in arbitrary order.
    pub fn values<'a>(&'a self) -> impl Iterator<Item = &NetSessionDevices> + 'a {
        self.session_code_to_id
            .right_values()
            .map(move |session_code_id| {
                self.session_id_to_device_mappings
                    .net_session_devices(*session_code_id)
                    .expect("Expected `NetSessionDevices` to exist.")
            })
    }

    /// Returns the number of sessions.
    ///
    /// Because each session has multiple devices, this will differ from the number of socket
    /// address mappings to the same session. For that, please use the [`len_devices`] method.
    pub fn len(&self) -> usize {
        self.session_id_to_device_mappings.len()
    }

    /// Returns the number of device to session code mappings.
    pub fn len_devices(&self) -> usize {
        self.session_id_to_device_mappings.len_devices()
    }
}

impl<'sdm> SessionDeviceMappings<'sdm> {
    /// Appends a new mapping from `SessionCode` to `NetSessionDevice`.
    ///
    /// If there is no existing entry for the session code, one will be created.
    ///
    /// # Parameters
    ///
    /// * `session_code`: Code of the session.
    /// * `net_session_device`: Device to append.
    pub fn append(&mut self, session_code: &SessionCode, net_session_device: NetSessionDevice) {
        let session_code_id = self.session_code_to_id.id_or_default(session_code);

        self.session_id_to_device_mappings
            .append(session_code_id, net_session_device);
    }

    /// Inserts a new mapping from `SessionCode` to `NetSessionDevices`.
    ///
    /// If a mapping previously existed for the session code, the devices are returned, but the key
    /// is not updated.
    ///
    /// If you are attempting to add another device to an existing session, please see the
    /// [`append`] method.
    ///
    /// # Parameters
    ///
    /// * `session_code`: Code of the session.
    /// * `net_session_devices`: Devices for the session.
    pub fn insert(
        &mut self,
        session_code: &SessionCode,
        net_session_devices: NetSessionDevices,
    ) -> Option<NetSessionDevices> {
        let session_code_id = self.session_code_to_id.id_or_default(session_code);

        self.session_id_to_device_mappings
            .insert(session_code_id, net_session_devices)
    }

    /// Removes the `NetSessionDevices` for the given `SessionCode`, returning it if present.
    pub fn remove(&mut self, session_code: &SessionCode) -> Option<NetSessionDevices> {
        let session_code_id = self
            .session_code_to_id
            .remove_by_left(session_code)
            .map(|(_session_code, session_code_id)| session_code_id);

        session_code_id
            .and_then(|session_code_id| self.session_id_to_device_mappings.remove(session_code_id))
    }

    /// Removes the device for the given `SocketAddr`, returning the `SessionCode` if it exists.
    pub fn remove_device(
        &mut self,
        socket_addr: &SocketAddr,
    ) -> Option<(&SessionCode, NetSessionDevice)> {
        let session_code_id_and_device = self
            .session_id_to_device_mappings
            .remove_device(socket_addr);

        session_code_id_and_device.and_then(move |(session_code_id, net_session_device)| {
            self.session_code_to_id
                .get_by_right(&session_code_id)
                .map(|session_code| (session_code, net_session_device))
        })
    }

    /// Removes the session and `NetSessionDevices` for the given `SocketAddr`, returning them if present.
    pub fn remove_session(
        &mut self,
        socket_addr: &SocketAddr,
    ) -> Option<(SessionCode, NetSessionDevices)> {
        self.session_id_to_device_mappings
            .session_code_id(socket_addr)
            .and_then(|session_code_id| {
                let session_code_and_id = self.session_code_to_id.remove_by_right(&session_code_id);
                let net_session_devices =
                    self.session_id_to_device_mappings.remove(session_code_id);

                if let (Some((session_code, _id)), Some(net_session_devices)) =
                    (session_code_and_id, net_session_devices)
                {
                    Some((session_code, net_session_devices))
                } else {
                    None
                }
            })
    }

    /// Reserves capacity for at least `additional` more mappings to be inserted.
    ///
    /// This may reserve more space to avosession_code frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    pub fn reserve(&mut self, additional: usize) {
        // `BiMap` does not provide a way to reserve capacity.
        // self.session_code_to_id.reserve(additional);
        self.session_id_to_device_mappings.reserve(additional);
    }

    pub fn as_read(&self) -> SessionDeviceMappingsRead<'_> {
        SessionDeviceMappingsRead::from(self)
    }
}

impl<'a, 'sdm: 'a> From<&'a SessionDeviceMappings<'sdm>> for SessionDeviceMappingsRead<'a> {
    fn from(session_device_mappings: &'a SessionDeviceMappings<'sdm>) -> Self {
        let SessionDeviceMappings {
            session_code_to_id,
            session_id_to_device_mappings,
        } = session_device_mappings;

        SessionDeviceMappingsRead {
            session_code_to_id,
            session_id_to_device_mappings,
        }
    }
}
