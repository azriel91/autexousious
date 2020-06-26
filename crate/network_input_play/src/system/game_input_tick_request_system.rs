use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use net_model::play::NetMessageEvent;
use network_session_model::{
    play::{SessionCondition, SessionStatus},
    SessionMessageEvent,
};

/// Informs the session server all client network input has been sent.
#[derive(Debug, new)]
pub struct GameInputTickRequestSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct GameInputTickRequestSystemData<'s> {
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `SessionCondition` resource.
    #[derivative(Debug = "ignore")]
    pub session_condition: Write<'s, SessionCondition>,
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
            mut session_condition,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        // Guard against sending input events if the application is not in a session.
        let session_established = *session_status == SessionStatus::JoinEstablished
            || *session_status == SessionStatus::HostEstablished;

        if session_established && *session_condition == SessionCondition::Ready {
            net_message_ec.single_write(NetMessageEvent::SessionMessageEvent(
                SessionMessageEvent::GameInputTick,
            ));
            *session_condition = SessionCondition::PendingGameInputTick;
        }
    }
}
