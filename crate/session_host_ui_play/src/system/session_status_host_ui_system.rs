use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::play::SessionStatus;
use session_host_model::SessionHostEvent;

/// Sends requests to a game server to host a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionStatusHostUiSystemDesc))]
pub struct SessionStatusHostUiSystem {
    /// Reader ID for the `SessionHostEvent` channel.
    #[system_desc(event_channel_reader)]
    session_host_event_rid: ReaderId<SessionHostEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionStatusHostUiSystemData<'s> {
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_ec: Read<'s, EventChannel<SessionHostEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
}

impl SessionStatusHostUiSystem {
    fn host_cancel_received<'s>(
        mut session_host_events: impl Iterator<Item = &'s SessionHostEvent>,
    ) -> bool {
        session_host_events.any(|ev| ev == &SessionHostEvent::HostCancel)
    }

    fn session_host_request_received<'s>(
        mut session_host_events: impl Iterator<Item = &'s SessionHostEvent>,
    ) -> bool {
        session_host_events.any(|ev| {
            if let SessionHostEvent::SessionHostRequest(_) = ev {
                true
            } else {
                false
            }
        })
    }
}

impl<'s> System<'s> for SessionStatusHostUiSystem {
    type SystemData = SessionStatusHostUiSystemData<'s>;

    fn run(
        &mut self,
        SessionStatusHostUiSystemData {
            session_host_ec,
            session_status,
        }: Self::SystemData,
    ) {
        let session_host_events = session_host_ec.read(&mut self.session_host_event_rid);

        match *session_status {
            SessionStatus::None | SessionStatus::HostEstablished => {
                let _should_remove_informative = Self::host_cancel_received(session_host_events);
                todo!("Issue #203");
            }
            SessionStatus::HostRequested => {
                let _should_create_informative =
                    Self::session_host_request_received(session_host_events);
                todo!("Issue #203");
            }
            _ => {}
        }

        //
    }
}
