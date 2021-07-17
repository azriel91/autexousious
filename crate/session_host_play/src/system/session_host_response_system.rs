use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::loaded::PlayerControllers;
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::play::{
    Session, SessionCode, SessionDeviceId, SessionDevices, SessionStatus,
};
use session_host_model::{play::SessionAcceptResponse, SessionHostEvent};

/// Records the session code and devices in the world when accepted into a
/// session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionHostResponseSystemDesc))]
pub struct SessionHostResponseSystem {
    /// Reader ID for the `SessionHostEvent` channel.
    #[system_desc(event_channel_reader)]
    session_host_event_rid: ReaderId<NetData<SessionHostEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionHostResponseSystemData<'s> {
    /// `SessionHostEvent` net channel.
    #[derivative(Debug = "ignore")]
    pub session_host_nec: Read<'s, NetEventChannel<SessionHostEvent>>,
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_ec: Write<'s, EventChannel<SessionHostEvent>>,
    /// `SessionCode` resource.
    #[derivative(Debug = "ignore")]
    pub session_code: Write<'s, SessionCode>,
    /// `SessionDeviceId` resource.
    #[derivative(Debug = "ignore")]
    pub session_device_id: Write<'s, SessionDeviceId>,
    /// `SessionDevices` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices: Write<'s, SessionDevices>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Write<'s, SessionStatus>,
    /// `PlayerControllers` resource.
    #[derivative(Debug = "ignore")]
    pub player_controllers: Write<'s, PlayerControllers>,
}

impl<'s> System<'s> for SessionHostResponseSystem {
    type SystemData = SessionHostResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionHostResponseSystemData {
            session_host_nec,
            mut session_host_ec,
            mut session_code,
            mut session_device_id,
            mut session_devices,
            mut session_status,
            mut player_controllers,
        }: Self::SystemData,
    ) {
        let session_host_events = session_host_nec.read(&mut self.session_host_event_rid);

        if let SessionStatus::HostRequested = &*session_status {
            // Use the last session response even if multiple are received.
            let session_status_new =
                session_host_events.fold(None, |mut session_status_new, ev| {
                    match ev {
                        NetData {
                            data: SessionHostEvent::SessionAccept(session_accept_response),
                            ..
                        } => {
                            debug!("Session accepted: {:?}", session_accept_response);

                            let SessionAcceptResponse {
                                session:
                                    Session {
                                        session_code: session_code_received,
                                        session_devices: session_devices_received,
                                    },
                                session_device_id: session_device_id_received,
                                player_controllers: player_controllers_received,
                            } = session_accept_response.clone();

                            // Write to resources.
                            *session_code = session_code_received;
                            *session_device_id = session_device_id_received;
                            *session_devices = session_devices_received;
                            session_status_new = Some(SessionStatus::HostEstablished);
                            *player_controllers = player_controllers_received;

                            session_host_ec.single_write(SessionHostEvent::SessionAccept(
                                session_accept_response.clone(),
                            ));
                        }
                        NetData {
                            data: SessionHostEvent::SessionReject(session_reject_response),
                            ..
                        } => {
                            debug!("Session rejected: {:?}", session_reject_response);

                            session_status_new = Some(SessionStatus::None);

                            session_host_ec.single_write(SessionHostEvent::SessionReject(
                                session_reject_response.clone(),
                            ));
                        }
                        _ => {}
                    }

                    session_status_new
                });

            if let Some(session_status_new) = session_status_new {
                *session_status = session_status_new;
            }
        }
    }
}
