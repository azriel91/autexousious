use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use net_model::play::NetMessageEvent;
use network_session_model::{play::SessionStatus, SessionMessageEvent};

/// Informs the session server all client network input has been sent.
#[derive(Debug, new)]
pub struct GameInputTickRequestSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GameInputTickRequestSystemData<'s> {
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `NetworkMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Write<'s, EventChannel<NetMessageEvent>>,
}

impl<'s> System<'s> for GameInputTickRequestSystem {
    type SystemData = GameInputTickRequestSystemData<'s>;

    fn run(
        &mut self,
        GameInputTickRequestSystemData {
            session_status,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        // Guard against sending input events if the application is not in a session.
        if *session_status == SessionStatus::JoinEstablished
            || *session_status == SessionStatus::HostEstablished
        {
            net_message_ec.single_write(NetMessageEvent::SessionMessageEvent(
                SessionMessageEvent::GameInputTick,
            ));
        }
    }
}
