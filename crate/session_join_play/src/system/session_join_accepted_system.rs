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
#[system_desc(name(SessionJoinAcceptedSystemDesc))]
pub struct SessionJoinAcceptedSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<SessionJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinAcceptedSystemData<'s> {
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

impl<'s> System<'s> for SessionJoinAcceptedSystem {
    type SystemData = SessionJoinAcceptedSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinAcceptedSystemData {
            session_join_ec,
            mut session_code,
            mut session_device_id,
            mut session_devices,
            mut session_status,
        }: Self::SystemData,
    ) {
        let mut session_join_events = session_join_ec.read(&mut self.session_join_event_rid);

        // Only process session accept event if user has not cancelled existing session join
        // request.
        if *session_status != SessionStatus::JoinRequested {
            return;
        }

        // Only process one session accept event if multiple are received.
        let session_accept_response = session_join_events.find_map(|ev| {
            if let SessionJoinEvent::SessionAccept(session_accept_response) = ev {
                Some(session_accept_response)
            } else {
                None
            }
        });

        if let Some(session_accept_response) = session_accept_response {
            debug!("Session accepted: {:?}", session_accept_response);

            // Write to resources.
            *session_code = session_accept_response.session_code.clone();
            *session_device_id = session_accept_response.session_device_id;
            *session_devices = session_accept_response.session_devices.clone();
            *session_status = SessionStatus::Established;
        }
    }
}
