use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::play::SessionStatus;
use session_lobby_model::SessionLobbyEvent;

/// Writes received `SessionLobbyEvent`s from the net channel to the regular event channel.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionLobbyResponseSystemDesc))]
pub struct SessionLobbyResponseSystem {
    /// Reader ID for the `SessionLobbyEvent` channel.
    #[system_desc(event_channel_reader)]
    session_lobby_event_rid: ReaderId<NetData<SessionLobbyEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionLobbyResponseSystemData<'s> {
    /// `SessionLobbyEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_lobby_nec: Read<'s, NetEventChannel<SessionLobbyEvent>>,
    /// `SessionLobbyEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_lobby_ec: Write<'s, EventChannel<SessionLobbyEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
}

impl<'s> System<'s> for SessionLobbyResponseSystem {
    type SystemData = SessionLobbyResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionLobbyResponseSystemData {
            session_lobby_nec,
            mut session_lobby_ec,
            session_status,
        }: Self::SystemData,
    ) {
        let session_lobby_events = session_lobby_nec.read(&mut self.session_lobby_event_rid);
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            session_lobby_events.for_each(|ev| match ev {
                NetData {
                    data: SessionLobbyEvent::SessionStartNotify,
                    ..
                } => {
                    debug!("Session start notification received.");

                    session_lobby_ec.single_write(SessionLobbyEvent::SessionStartNotify);
                }
                _ => {}
            });
        }
    }
}
