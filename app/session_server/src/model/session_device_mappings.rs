use std::{collections::HashMap, convert::TryInto, net::SocketAddr};

use bimap::BiMap;
use derive_new::new;
use net_model::play::{NetSessionDevice, NetSessionDevices};
use network_session_model::play::SessionCode;

/// ID so that we don't have to clone `SessionCode`.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct SessionCodeId(pub u64);

/// Mappings from `SessionCode` to `NetSessionDevices`, and `SocketAddr` to `SessionCode`.
#[derive(Clone, Debug, Default, new)]
pub struct SessionDeviceMappings {
    /// Mappings from `SessionCode` to `SessionCodeId`.
    #[new(default)]
    session_code_to_id: BiMap<SessionCode, SessionCodeId>,
    /// Mappings from `SessionCodeId` to `NetSessionDevices`.
    #[new(default)]
    session_code_id_to_devices: HashMap<SessionCodeId, NetSessionDevices>,
    /// Mappings from `SocketAddr` to `SessionCodeId`.
    #[new(default)]
    socket_addr_to_session_code_id: HashMap<SocketAddr, SessionCodeId>,
}

impl SessionDeviceMappings {
    /// Returns a `SessionDeviceMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        SessionDeviceMappings {
            session_code_to_id: BiMap::with_capacity(capacity),
            session_code_id_to_devices: HashMap::with_capacity(capacity),
            socket_addr_to_session_code_id: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the mappings can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.session_code_id_to_devices.capacity()
    }

    /// Returns the `NetSessionDevices` for the given `SessionCode`.
    pub fn net_session_devices(&self, session_code: &SessionCode) -> Option<&NetSessionDevices> {
        self.session_code_to_id
            .get_by_left(session_code)
            .and_then(|session_code_id| self.session_code_id_to_devices.get(session_code_id))
    }

    /// Returns the `SessionCode` for the given `SocketAddr`.
    pub fn session_code(&self, socket_addr: &SocketAddr) -> Option<&SessionCode> {
        self.socket_addr_to_session_code_id
            .get(socket_addr)
            .and_then(|session_code_id| self.session_code_to_id.get_by_right(session_code_id))
    }

    /// Returns `true` if there are no sessions.
    pub fn is_empty(&self) -> bool {
        self.session_code_to_id.is_empty()
    }

    /// Appends a new mapping from `SessionCode` to `NetSessionDevice`.
    ///
    /// If there is no existing entry for the session code, one will be created.
    ///
    /// # Parameters
    ///
    /// * `session_code`: Code of the session.
    /// * `net_session_device`: Device to append.
    pub fn append(&mut self, session_code: &SessionCode, net_session_device: NetSessionDevice) {
        let session_code_id = self
            .session_code_to_id
            .get_by_left(session_code)
            .copied()
            .unwrap_or_else(|| {
                let session_code_id: u64 = self
                    .session_code_to_id
                    .len()
                    .try_into()
                    .expect("Failed to convert `usize` to `u64` for `SessionCodeId`.");
                SessionCodeId(session_code_id)
            });

        let net_session_devices = self
            .session_code_id_to_devices
            .entry(session_code_id)
            .or_insert_with(NetSessionDevices::default);

        // Register `SocketAddr`
        self.socket_addr_to_session_code_id
            .insert(net_session_device.socket_addr, session_code_id);
        net_session_devices.push(net_session_device);
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
        let session_code_id = self
            .session_code_to_id
            .get_by_left(session_code)
            .copied()
            .unwrap_or_else(|| {
                let session_code_id: u64 = self
                    .session_code_to_id
                    .len()
                    .try_into()
                    .expect("Failed to convert `usize` to `u64` for `SessionCodeId`.");

                let session_code_id = SessionCodeId(session_code_id);

                self.session_code_to_id
                    .insert(session_code.clone(), session_code_id);

                session_code_id
            });

        let mut existing_devices = self
            .session_code_id_to_devices
            .insert(session_code_id, net_session_devices);

        // === Update `SocketAddr` mappings. === //
        // Remove existing mappings
        if let Some(existing_devices) = existing_devices.as_mut() {
            existing_devices.iter().for_each(|net_session_device| {
                self.socket_addr_to_session_code_id
                    .remove(&net_session_device.socket_addr);
            });
        }

        let SessionDeviceMappings {
            session_code_id_to_devices,
            socket_addr_to_session_code_id,
            ..
        } = self;

        // Add new mappings
        if let Some(net_session_devices) = session_code_id_to_devices.get(&session_code_id) {
            net_session_devices.iter().for_each(|net_session_device| {
                socket_addr_to_session_code_id
                    .insert(net_session_device.socket_addr, session_code_id);
            });
        }

        existing_devices
    }

    /// Returns an iterator of `SessionCode`s to `NetSessionDevices`.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&SessionCode, &NetSessionDevices)> + 'a {
        self.session_code_to_id
            .iter()
            .map(move |(session_code, session_code_id)| {
                let net_session_devices = self
                    .session_code_id_to_devices
                    .get(session_code_id)
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
                self.session_code_id_to_devices
                    .get(session_code_id)
                    .expect("Expected `NetSessionDevices` to exist.")
            })
    }

    /// Returns the number of sessions.
    ///
    /// Because each session has multiple devices, this will differ from the number of socket
    /// address mappings to the same session. For that, please use the [`len_devices`] method.
    pub fn len(&self) -> usize {
        self.session_code_id_to_devices.len()
    }

    /// Returns the number of device to session code mappings.
    pub fn len_devices(&self) -> usize {
        self.socket_addr_to_session_code_id.len()
    }

    /// Removes the `NetSessionDevices` for the given `SessionCode`, returning it if present.
    pub fn remove(&mut self, session_code: &SessionCode) -> Option<NetSessionDevices> {
        let session_code_id = self
            .session_code_to_id
            .remove_by_left(session_code)
            .map(|(_session_code, session_code_id)| session_code_id);

        session_code_id.and_then(|session_code_id| {
            Self::remove_session_devices(
                &mut self.session_code_id_to_devices,
                &mut self.socket_addr_to_session_code_id,
                session_code_id,
            )
        })
    }

    /// Removes the `NetSessionDevices` for the given `SessionCodeId`, returning it if present.
    fn remove_session_devices(
        session_code_id_to_devices: &mut HashMap<SessionCodeId, NetSessionDevices>,
        socket_addr_to_session_code_id: &mut HashMap<SocketAddr, SessionCodeId>,
        session_code_id: SessionCodeId,
    ) -> Option<NetSessionDevices> {
        let mut net_session_devices = session_code_id_to_devices.remove(&session_code_id);

        // === Update `SocketAddr` mappings. === //
        // Remove existing mappings
        if let Some(net_session_devices) = net_session_devices.as_mut() {
            net_session_devices.iter().for_each(|net_session_device| {
                socket_addr_to_session_code_id.remove(&net_session_device.socket_addr);
            });
        }

        net_session_devices
    }

    /// Removes the device for the given `SocketAddr`, returning the session code it if it exists.
    pub fn remove_device(&mut self, socket_addr: &SocketAddr) -> Option<&SessionCode> {
        let session_code_id = self.socket_addr_to_session_code_id.remove(socket_addr);

        if let Some(session_code_id) = session_code_id {
            Self::remove_session_device(
                &mut self.session_code_id_to_devices,
                &mut self.socket_addr_to_session_code_id,
                session_code_id,
                *socket_addr,
            );
        }

        session_code_id
            .as_ref()
            .and_then(move |session_code_id| self.session_code_to_id.get_by_right(session_code_id))
    }

    /// Removes the `NetSessionDevice` for the given `SocketAddr`, returning it if present.
    fn remove_session_device(
        session_code_id_to_devices: &mut HashMap<SessionCodeId, NetSessionDevices>,
        socket_addr_to_session_code_id: &mut HashMap<SocketAddr, SessionCodeId>,
        session_code_id: SessionCodeId,
        socket_addr: SocketAddr,
    ) -> Option<NetSessionDevice> {
        let mut net_session_devices = session_code_id_to_devices.get_mut(&session_code_id);

        // === Update `SocketAddr` mappings. === //
        // Remove existing mappings
        if let Some(net_session_devices) = net_session_devices.as_mut() {
            socket_addr_to_session_code_id.remove(&socket_addr);

            let net_session_device_index =
                net_session_devices
                    .iter()
                    .enumerate()
                    .find_map(|(index, net_session_device)| {
                        if net_session_device.socket_addr == socket_addr {
                            Some(index)
                        } else {
                            None
                        }
                    });

            net_session_device_index.map(|net_session_device_index| {
                net_session_devices.swap_remove(net_session_device_index)
            })
        } else {
            None
        }
    }

    /// Reserves capacity for at least `additional` more mappings to be inserted.
    ///
    /// This may reserve more space to avosession_code frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    pub fn reserve(&mut self, additional: usize) {
        self.session_code_id_to_devices.reserve(additional);
        self.socket_addr_to_session_code_id.reserve(additional);
    }
}
