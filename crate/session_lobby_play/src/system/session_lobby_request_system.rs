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
use session_lobby_model::SessionLobbyEvent;

/// Sends requests to a game server to lobby a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionLobbyRequestSystemDesc))]
pub struct SessionLobbyRequestSystem {
    /// Reader ID for the `SessionLobbyEvent` channel.
    #[system_desc(event_channel_reader)]
    session_lobby_event_rid: ReaderId<SessionLobbyEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionLobbyRequestSystemData<'s> {
    /// `SessionLobbyEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_lobby_ec: Read<'s, EventChannel<SessionLobbyEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `NetworkMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Write<'s, EventChannel<NetMessageEvent>>,
}

impl<'s> System<'s> for SessionLobbyRequestSystem {
    type SystemData = SessionLobbyRequestSystemData<'s>;

    fn run(
        &mut self,
        SessionLobbyRequestSystemData {
            session_lobby_ec,
            session_status,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        let mut session_lobby_events = session_lobby_ec.read(&mut self.session_lobby_event_rid);

        // Guard against requesting session to start if the application is not in a session.
        if *session_status == SessionStatus::JoinEstablished
            || *session_status == SessionStatus::HostEstablished
        {
            // Only process one session lobby request event if multiple are received.
            let session_start_request_params = session_lobby_events.find_map(|ev| {
                if let SessionLobbyEvent::SessionStartRequest(session_start_request_params) = ev {
                    Some(session_start_request_params)
                } else {
                    None
                }
            });

            if let Some(session_start_request_params) = session_start_request_params {
                net_message_ec.single_write(NetMessageEvent::SessionLobbyEvent(
                    SessionLobbyEvent::SessionStartRequest(session_start_request_params.clone()),
                ));
            }
        }
    }
}
