use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use network_join_model::NetworkJoinEvent;

/// Records the session code and devices in the world when accepted into a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionJoinAcceptedSystemDesc))]
pub struct SessionJoinAcceptedSystem {
    /// Reader ID for the `NetworkJoinEvent` channel.
    #[system_desc(event_channel_reader)]
    network_join_event_rid: ReaderId<NetworkJoinEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionJoinAcceptedSystemData<'s> {
    /// `NetworkJoinEvent` channel.
    #[derivative(Debug = "ignore")]
    pub network_join_ec: Read<'s, EventChannel<NetworkJoinEvent>>,
}

impl<'s> System<'s> for SessionJoinAcceptedSystem {
    type SystemData = SessionJoinAcceptedSystemData<'s>;

    fn run(&mut self, SessionJoinAcceptedSystemData { network_join_ec }: Self::SystemData) {
        // Only process one session accept event if multiple are received.
        let session_accept_response = network_join_ec
            .read(&mut self.network_join_event_rid)
            .find_map(|ev| {
                if let NetworkJoinEvent::SessionAccept(session_accept_response) = ev {
                    Some(session_accept_response)
                } else {
                    None
                }
            });

        if let Some(session_accept_response) = session_accept_response {
            debug!("Session accepted: {:?}", session_accept_response);

            // TODO: write to resources.
        }
    }
}
