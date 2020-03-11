use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use net_model::play::NetMessageEvent;
use network_session_model::play::SessionStatus;
use session_host_model::SessionHostEvent;

/// Sends requests to a game server to host a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionHostRequestSystemDesc))]
pub struct SessionHostRequestSystem {
    /// Reader ID for the `SessionHostEvent` channel.
    #[system_desc(event_channel_reader)]
    session_host_event_rid: ReaderId<SessionHostEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionHostRequestSystemData<'s> {
    /// `SessionHostEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_host_ec: Read<'s, EventChannel<SessionHostEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Write<'s, SessionStatus>,
    /// `NetworkMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Write<'s, EventChannel<NetMessageEvent>>,
}

impl<'s> System<'s> for SessionHostRequestSystem {
    type SystemData = SessionHostRequestSystemData<'s>;

    fn run(
        &mut self,
        SessionHostRequestSystemData {
            session_host_ec,
            mut session_status,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        let mut session_host_events = session_host_ec.read(&mut self.session_host_event_rid);

        // Guard against requesting multiple sessions at the same time.
        if *session_status != SessionStatus::None {
            return;
        }

        // Only process one session host event if multiple are received.
        let session_host_request_params = session_host_events.find_map(|ev| {
            if let SessionHostEvent::SessionHostRequest(session_host_request_params) = ev {
                Some(session_host_request_params)
            } else {
                None
            }
        });

        if let Some(session_host_request_params) = session_host_request_params {
            net_message_ec.single_write(NetMessageEvent::SessionHostEvent(
                SessionHostEvent::SessionHostRequest(session_host_request_params.clone()),
            ));

            *session_status = SessionStatus::HostRequested;
        }
    }
}
