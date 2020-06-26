use std::{collections::HashMap, net::SocketAddr};

use derive_new::new;
use net_model::play::{NetSessionDevice, NetSessionDevices};

use crate::model::SessionCodeId;

/// Mappings from `SessionCodeId` to `NetSessionDevices`, and `SocketAddr` to `SessionCodeId`.
#[derive(Clone, Debug, Default, new)]
pub struct SessionIdToDeviceMappings {
    /// Mappings from `SessionCodeId` to `NetSessionDevices`.
    #[new(default)]
    session_code_id_to_devices: HashMap<SessionCodeId, NetSessionDevices>,
    /// Mappings from `SocketAddr` to `SessionCodeId`.
    #[new(default)]
    socket_addr_to_session_code_id: HashMap<SocketAddr, SessionCodeId>,
}

impl SessionIdToDeviceMappings {
    /// Returns a `SessionIdToDeviceMappings` with pre-allocated capacity.
    ///
    /// The mappings are guaranteed to hold `capacity` elements without re-allocating.
    pub fn with_capacity(capacity: usize) -> Self {
        SessionIdToDeviceMappings {
            session_code_id_to_devices: HashMap::with_capacity(capacity),
            socket_addr_to_session_code_id: HashMap::with_capacity(capacity),
        }
    }

    /// Returns the number of elements the mappings can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.session_code_id_to_devices.capacity()
    }

    /// Returns the `NetSessionDevices` for the given `SessionCodeId`.
    pub fn net_session_devices(
        &self,
        session_code_id: SessionCodeId,
    ) -> Option<&NetSessionDevices> {
        self.session_code_id_to_devices.get(&session_code_id)
    }

    /// Returns the `SessionCodeId` for the given `SocketAddr`.
    pub fn session_code_id(&self, socket_addr: &SocketAddr) -> Option<SessionCodeId> {
        self.socket_addr_to_session_code_id
            .get(socket_addr)
            .copied()
    }

    /// Returns `true` if there are no sessions.
    pub fn is_empty(&self) -> bool {
        self.session_code_id_to_devices.is_empty()
    }

    /// Appends a new mapping from `SessionCodeId` to `NetSessionDevice`.
    ///
    /// If there is no existing entry for the session code, one will be created.
    ///
    /// # Parameters
    ///
    /// * `session_code_id`: ID of the session.
    /// * `net_session_device`: Device to append.
    pub fn append(&mut self, session_code_id: SessionCodeId, net_session_device: NetSessionDevice) {
        let net_session_devices = self
            .session_code_id_to_devices
            .entry(session_code_id)
            .or_insert_with(NetSessionDevices::default);

        // Register `SocketAddr`
        self.socket_addr_to_session_code_id
            .insert(net_session_device.socket_addr, session_code_id);
        net_session_devices.push(net_session_device);
    }

    /// Inserts a new mapping from `SessionCodeId` to `NetSessionDevices`.
    ///
    /// If a mapping previously existed for the session code, the devices are returned, but the key
    /// is not updated.
    ///
    /// If you are attempting to add another device to an existing session, please see the
    /// [`append`] method.
    ///
    /// # Parameters
    ///
    /// * `session_code_id`: ID of the session.
    /// * `net_session_devices`: Devices for the session.
    pub fn insert(
        &mut self,
        session_code_id: SessionCodeId,
        net_session_devices: NetSessionDevices,
    ) -> Option<NetSessionDevices> {
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

        let SessionIdToDeviceMappings {
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

    /// Returns an iterator of `SessionCodeId`s to `NetSessionDevices` in arbitrary order.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&SessionCodeId, &NetSessionDevices)> + 'a {
        self.session_code_id_to_devices.iter()
    }

    /// Returns an iterator visiting all `SessionCodeId`s in arbitrary order.
    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &SessionCodeId> + 'a {
        self.session_code_id_to_devices.keys()
    }

    /// Returns an iterator visiting all `NetSessionDevices` in arbitrary order.
    pub fn values<'a>(&'a self) -> impl Iterator<Item = &NetSessionDevices> + 'a {
        self.session_code_id_to_devices.values()
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

    /// Removes the `NetSessionDevices` for the given `SessionCodeId`, returning it if present.
    pub fn remove(&mut self, session_code_id: SessionCodeId) -> Option<NetSessionDevices> {
        let mut net_session_devices = self.session_code_id_to_devices.remove(&session_code_id);

        // === Update `SocketAddr` mappings. === //
        // Remove existing mappings
        if let Some(net_session_devices) = net_session_devices.as_mut() {
            net_session_devices.iter().for_each(|net_session_device| {
                self.socket_addr_to_session_code_id
                    .remove(&net_session_device.socket_addr);
            });
        }

        net_session_devices
    }

    /// Removes the device for the given `SocketAddr`, returning it with the `SessionCodeId` if it exists.
    pub fn remove_device(
        &mut self,
        socket_addr: &SocketAddr,
    ) -> Option<(SessionCodeId, NetSessionDevice)> {
        let session_code_id = self.socket_addr_to_session_code_id.remove(socket_addr);

        if let Some(session_code_id) = session_code_id {
            let net_session_device = Self::remove_device_internal(
                &mut self.session_code_id_to_devices,
                &mut self.socket_addr_to_session_code_id,
                session_code_id,
                *socket_addr,
            );

            if let Some(net_session_device) = net_session_device {
                Some((session_code_id, net_session_device))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Removes the `NetSessionDevice` for the given `SocketAddr`, returning it if present.
    fn remove_device_internal(
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
