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
use session_join_model::SessionJoinEvent;

/// Sends requests to a game server to join a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinRequestSystemDesc))]
pub struct SessionJoinRequestSystem {
    /// Reader ID for the `SessionJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    session_join_event_rid: ReaderId<SessionJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinRequestSystemData<'s> {
    /// `SessionJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_join_ec: Read<'s, EventChannel<SessionJoinEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Write<'s, SessionStatus>,
    /// `NetworkMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Write<'s, EventChannel<NetMessageEvent>>,
}

impl<'s> System<'s> for SessionJoinRequestSystem {
    type SystemData = SessionJoinRequestSystemData<'s>;

    fn run(
        &mut self,
        SessionJoinRequestSystemData {
            session_join_ec,
            mut session_status,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        let mut session_join_events = session_join_ec.read(&mut self.session_join_event_rid);

        // Guard against requesting multiple sessions at the same time.
        if *session_status != SessionStatus::None {
            return;
        }

        // Only process one session join event if multiple are received.
        let session_join_request_params = session_join_events.find_map(|ev| {
            if let SessionJoinEvent::SessionJoinRequest(session_join_request_params) = ev {
                Some(session_join_request_params)
            } else {
                None
            }
        });

        if let Some(session_join_request_params) = session_join_request_params {
            net_message_ec.single_write(NetMessageEvent::SessionJoinEvent(
                SessionJoinEvent::SessionJoinRequest(session_join_request_params.clone()),
            ));

            *session_status = SessionStatus::JoinRequested {
                session_code: session_join_request_params.session_code.clone(),
            };
        }
    }
}
