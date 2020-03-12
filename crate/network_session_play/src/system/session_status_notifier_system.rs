use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::{play::SessionStatus, SessionStatusEvent};
use tracker::Prev;

/// Send a `SessionStatusEvent` when the `SessionStatus` resource changes.
#[derive(Debug, new)]
pub struct SessionStatusNotifierSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionStatusNotifierSystemData<'s> {
    /// Previous `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status_prev: Write<'s, Prev<SessionStatus>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `SessionStatusEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_status_ec: Write<'s, EventChannel<SessionStatusEvent>>,
}

impl<'s> System<'s> for SessionStatusNotifierSystem {
    type SystemData = SessionStatusNotifierSystemData<'s>;

    fn run(
        &mut self,
        SessionStatusNotifierSystemData {
            mut session_status_prev,
            session_status,
            mut session_status_ec,
        }: Self::SystemData,
    ) {
        if session_status_prev.0 != *session_status {
            session_status_ec.single_write(SessionStatusEvent);
            session_status_prev.0 = session_status.clone();
        }
    }
}
