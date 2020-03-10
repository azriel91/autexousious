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
use session_join_model::{play::SessionAcceptResponse, SessionJoinEvent};

/// Records the session code and devices in the world when accepted into a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinResponseSystemDesc))]
pub struct SessionJoinResponseSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<NetData<SessionJoinEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinResponseSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_nec: Read<'s, NetEventChannel<SessionJoinEvent>>,
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Write<'s, EventChannel<SessionJoinEvent>>,
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

impl<'s> System<'s> for SessionJoinResponseSystem {
    type SystemData = SessionJoinResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinResponseSystemData {
            session_join_nec,
            mut session_join_ec,
            mut session_code,
            mut session_device_id,
            mut session_devices,
            mut session_status,
            mut player_controllers,
        }: Self::SystemData,
    ) {
        let session_join_events = session_join_nec.read(&mut self.session_join_event_rid);

        if let SessionStatus::JoinRequested {
            session_code: session_code_requested,
        } = &*session_status
        {
            // Use the last session response even if multiple are received.
            let session_status_new =
                session_join_events.fold(None, |mut session_status_new, ev| {
                    match ev {
                        NetData {
                            data: SessionJoinEvent::SessionAccept(session_accept_response),
                            ..
                        } if &session_accept_response.session.session_code
                            == session_code_requested =>
                        {
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
                            session_status_new = Some(SessionStatus::JoinEstablished);
                            *player_controllers = player_controllers_received;

                            session_join_ec.single_write(SessionJoinEvent::SessionAccept(
                                session_accept_response.clone(),
                            ));
                        }
                        NetData {
                            data: SessionJoinEvent::SessionReject(session_reject_response),
                            ..
                        } if &session_reject_response.session_code == session_code_requested => {
                            debug!("Session rejected: {:?}", session_reject_response);

                            session_status_new = Some(SessionStatus::None);

                            session_join_ec.single_write(SessionJoinEvent::SessionReject(
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
