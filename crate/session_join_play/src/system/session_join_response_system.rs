use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use network_session_model::play::{SessionCode, SessionDeviceId, SessionDevices, SessionStatus};
use session_join_model::SessionJoinEvent;

/// Records the session code and devices in the world when accepted into a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinResponseSystemDesc))]
pub struct SessionJoinResponseSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<SessionJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinResponseSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Read<'s, EventChannel<SessionJoinEvent>>,
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
}

impl<'s> System<'s> for SessionJoinResponseSystem {
    type SystemData = SessionJoinResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinResponseSystemData {
            session_join_ec,
            mut session_code,
            mut session_device_id,
            mut session_devices,
            mut session_status,
        }: Self::SystemData,
    ) {
        let session_join_events = session_join_ec.read(&mut self.session_join_event_rid);

        if let SessionStatus::JoinRequested {
            session_code: session_code_requested,
        } = &*session_status
        {
            // Use the last session response even if multiple are received.
            let session_status_new =
                session_join_events.fold(None, |mut session_status_new, ev| {
                    match ev {
                        SessionJoinEvent::SessionAccept(session_accept_response)
                            if &session_accept_response.session.session_code
                                == session_code_requested =>
                        {
                            debug!("Session accepted: {:?}", session_accept_response);

                            // Write to resources.
                            *session_code = session_accept_response.session.session_code.clone();
                            *session_device_id = session_accept_response.session_device_id;
                            *session_devices =
                                session_accept_response.session.session_devices.clone();
                            session_status_new = Some(SessionStatus::Established);
                        }
                        SessionJoinEvent::SessionReject(session_reject_response)
                            if &session_reject_response.session_code == session_code_requested =>
                        {
                            debug!("Session rejected: {:?}", session_reject_response);

                            session_status_new = Some(SessionStatus::None);
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
