use std::net::SocketAddr;

use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    network::simulation::{DeliveryRequirement, TransportResource, UrgencyRequirement},
    shred::{ResourceId, SystemData},
    shrev::ReaderId,
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error, warn};
use net_model::play::{NetData, NetEventChannel, NetMessageEvent};
use network_session_model::{
    play::{SessionDeviceId, SessionDeviceName},
    SessionMessageEvent,
};

use crate::model::{
    GameInputTickStatus, SessionCodeId, SessionCodeToId, SessionDeviceTickStatuses,
    SessionIdToDeviceMappings, SessionTickStatuses, SocketToDeviceId,
};

/// Notifies game clients when all `GameInputEvent`s have been sent for the current tick.
///
/// When `SessionMessageEvent::GameInputTick` has been received from each client, the session server then sends its own
/// `SessionMessageEvent::GameInputTick` messages to all clients, notifying them that all `GameInputEvent`s have been
/// sent to them.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionMessageResponderSystemDesc))]
pub struct SessionMessageResponderSystem {
    /// Reader ID for the `SessionMessageEvent` channel.
    #[system_desc(event_channel_reader)]
    session_message_event_rid: ReaderId<NetData<SessionMessageEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionMessageResponderSystemData<'s> {
    /// `SessionMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_message_nec: Read<'s, NetEventChannel<SessionMessageEvent>>,
    /// `SessionIdToDeviceMappings` resource.
    #[derivative(Debug = "ignore")]
    pub session_id_to_device_mappings: Read<'s, SessionIdToDeviceMappings>,
    /// `SocketToDeviceId` resource.
    #[derivative(Debug = "ignore")]
    pub socket_to_device_id: Read<'s, SocketToDeviceId>,
    /// `SessionCodeToId` resource.
    #[derivative(Debug = "ignore")]
    pub session_code_to_id: Read<'s, SessionCodeToId>,
    /// `SessionTickStatuses` resource.
    #[derivative(Debug = "ignore")]
    pub session_tick_statuses: Write<'s, SessionTickStatuses>,
    /// `TransportResource` resource.
    #[derivative(Debug = "ignore")]
    pub transport_resource: Write<'s, TransportResource>,
}

impl SessionMessageResponderSystem {
    /// Returns the `SessionCodeId` and `SessionDeviceTickStatuses` associated with the event's client.
    ///
    /// This log a warning if the `SessionCodeId` or `SessionDeviceTickStatuses` cannot be found.
    fn select_session_information<'tick>(
        session_id_to_device_mappings: &SessionIdToDeviceMappings,
        socket_to_device_id: &SocketToDeviceId,
        session_tick_statuses: &'tick mut SessionTickStatuses,
        net_session_message_event: &NetData<SessionMessageEvent>,
    ) -> Result<
        (
            &'tick mut SessionDeviceTickStatuses,
            SessionCodeId,
            SessionDeviceId,
        ),
        String,
    > {
        let NetData {
            socket_addr,
            data: session_message_event,
        } = net_session_message_event;

        let session_code_id = session_id_to_device_mappings
            .session_code_id(&socket_addr)
            .ok_or_else(|| {
                // TODO: reject
                format!(
                    "Received `{:?}` from {}, but no session code tracked for that socket.",
                    session_message_event, socket_addr
                )
            })?;

        let session_device_tick_statuses = session_tick_statuses
            .entry(session_code_id)
            .or_insert_with(SessionDeviceTickStatuses::default);

        let session_device_id =
            socket_to_device_id
                .get(&socket_addr)
                .copied()
                .ok_or_else(|| {
                    format!(
                    "Received `{:?}` from {}, but no `SessionDeviceId` tracked for that socket.",
                    session_message_event, socket_addr
                )
                })?;

        Ok((
            session_device_tick_statuses,
            session_code_id,
            session_device_id,
        ))
    }

    /// Returns the `SessionDeviceName` for the given `SessionDeviceId`.
    fn session_device_name(
        session_id_to_device_mappings: &SessionIdToDeviceMappings,
        session_code_id: SessionCodeId,
        session_device_id: SessionDeviceId,
    ) -> Option<&SessionDeviceName> {
        let net_session_devices =
            session_id_to_device_mappings.net_session_devices(session_code_id);
        net_session_devices.and_then(|net_session_devices| {
            net_session_devices.iter().find_map(|net_session_device| {
                if net_session_device.data.id == session_device_id {
                    Some(&net_session_device.data.name)
                } else {
                    None
                }
            })
        })
    }

    /// Stores `GameInputTickStatus::Received` for the given `SessionDeviceId`.
    fn tick_device(
        session_id_to_device_mappings: &SessionIdToDeviceMappings,
        session_device_tick_statuses: &mut SessionDeviceTickStatuses,
        socket_addr: SocketAddr,
        session_code_id: SessionCodeId,
        session_device_id: SessionDeviceId,
    ) {
        let game_input_tick_status_previous =
            session_device_tick_statuses.insert(session_device_id, GameInputTickStatus::Received);
        if let Some(GameInputTickStatus::Received) = game_input_tick_status_previous {
            let device_name = Self::session_device_name(
                session_id_to_device_mappings,
                session_code_id,
                session_device_id,
            );
            if let Some(device_name) = device_name {
                warn!(
                    "Received `SessionMessageEvent::GameInputTick` twice from {}.",
                    device_name
                );
            } else {
                warn!(
                    "Received `SessionMessageEvent::GameInputTick` twice from {}.",
                    socket_addr
                );
            }
        }
    }

    /// Returns whether `SessionMessageEvent::GameInputTick` has been received from all devices in the session.
    fn session_tick_all_received(session_device_tick_statuses: &SessionDeviceTickStatuses) -> bool {
        session_device_tick_statuses
            .values()
            .all(|game_input_tick_status| *game_input_tick_status == GameInputTickStatus::Received)
    }

    /// Sends `SessionMessageEvent::GameInputTick` to all devices in a `Session`.
    fn session_game_input_tick(
        session_id_to_device_mappings: &SessionIdToDeviceMappings,
        session_code_to_id: &SessionCodeToId,
        transport_resource: &mut TransportResource,
        session_code_id: SessionCodeId,
    ) {
        let session_code = session_code_to_id.code(session_code_id);
        if let Some(session_code) = session_code {
            debug!(
                "Sending `SessionMessageEvent::GameInputTick` for session: `{}`.",
                session_code
            );
        } else {
            debug!(
                "Sending `SessionMessageEvent::GameInputTick` for session: `{}`.",
                session_code_id
            );
        }

        let net_session_devices =
            session_id_to_device_mappings.net_session_devices(session_code_id);
        if let Some(net_session_devices) = net_session_devices {
            let socket_addrs = net_session_devices
                .iter()
                .map(|net_session_device| net_session_device.socket_addr);

            Self::send_game_input_tick(transport_resource, socket_addrs);
        } else if let Some(session_code) = session_code {
            error!(
                "`NetSessionDevices` not found for session: `{}`",
                session_code
            );
        } else {
            error!(
                "`NetSessionDevices` not found for `SessionCodeId`: `{}`",
                session_code_id
            );
        }
    }

    fn send_game_input_tick(
        transport_resource: &mut TransportResource,
        socket_addrs: impl Iterator<Item = SocketAddr>,
    ) {
        let net_message_event = NetMessageEvent::from(SessionMessageEvent::GameInputTick);

        match bincode::serialize(&net_message_event) {
            Ok(payload) => {
                socket_addrs.for_each(|socket_addr| {
                    transport_resource.send_with_requirements(
                        socket_addr,
                        &payload,
                        // None means it uses a default multiplexed stream.
                        //
                        // Suspect if we give it a value, the value will be a "channel" over the same
                        // socket connection.
                        DeliveryRequirement::ReliableOrdered(None),
                        UrgencyRequirement::OnTick,
                    );
                });
            }
            Err(e) => {
                error!(
                    "Failed to serialize `NetMessageEvent::InputEvent`. Error: `{}`.",
                    e
                );
            }
        }
    }
}

impl<'s> System<'s> for SessionMessageResponderSystem {
    type SystemData = SessionMessageResponderSystemData<'s>;

    fn run(
        &mut self,
        SessionMessageResponderSystemData {
            session_message_nec,
            session_id_to_device_mappings,
            socket_to_device_id,
            session_code_to_id,
            mut session_tick_statuses,
            mut transport_resource,
        }: Self::SystemData,
    ) {
        // 1. When a client sends `SessionMessageEvent::GameInputTick`, record it.
        // 2. When all clients have sent `GameInputTick`, send `GameInputTick` to all of them.
        //
        // Error cases:
        //
        // * If the client has sent `GameInputTick` more than once, ignore it and log a warning.
        // * If the client isn't registered to a session, ignore it and log a warning.

        let session_id_to_device_mappings = &session_id_to_device_mappings;
        let session_tick_statuses = &mut session_tick_statuses;
        session_message_nec
            .read(&mut self.session_message_event_rid)
            .filter(|net_session_message_event| {
                net_session_message_event.data == SessionMessageEvent::GameInputTick
            })
            .for_each(|net_session_message_event| {
                let session_information = Self::select_session_information(
                    session_id_to_device_mappings,
                    &socket_to_device_id,
                    session_tick_statuses,
                    net_session_message_event,
                );

                let NetData { socket_addr, .. } = net_session_message_event;

                match session_information {
                    Ok((session_device_tick_statuses, session_code_id, session_device_id)) => {
                        // Record `GameInputTick` for this client.
                        Self::tick_device(
                            session_id_to_device_mappings,
                            session_device_tick_statuses,
                            *socket_addr,
                            session_code_id,
                            session_device_id,
                        );

                        // When all devices are ready, send `GameInputTick`
                        let session_tick_all_received =
                            Self::session_tick_all_received(session_device_tick_statuses);
                        if session_tick_all_received {
                            Self::session_game_input_tick(
                                session_id_to_device_mappings,
                                &session_code_to_id,
                                &mut transport_resource,
                                session_code_id,
                            );

                            // Reset session device tick statuses.
                            session_device_tick_statuses
                                .values_mut()
                                .for_each(|tick_status| {
                                    *tick_status = GameInputTickStatus::Pending
                                });
                        }
                    }
                    Err(e) => warn!("{}", e),
                }
            });
    }
}
