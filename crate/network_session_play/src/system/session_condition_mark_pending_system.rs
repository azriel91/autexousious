use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use network_session_model::play::{SessionCondition, SessionStatus};

/// Sets the `SessionCondition` to `SessionCondition::PendingGameInputTick` when a session is established.
#[derive(Debug, new)]
pub struct SessionConditionMarkPendingSystem;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionConditionMarkPendingSystemData<'s> {
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `SessionCondition` resource.
    #[derivative(Debug = "ignore")]
    pub session_condition: Write<'s, SessionCondition>,
}

impl<'s> System<'s> for SessionConditionMarkPendingSystem {
    type SystemData = SessionConditionMarkPendingSystemData<'s>;

    fn run(
        &mut self,
        SessionConditionMarkPendingSystemData {
            session_status,
            mut session_condition,
        }: Self::SystemData,
    ) {
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            // Wait for all `GameInputEvent`s from the session server.
            *session_condition = SessionCondition::PendingGameInputTick;
        } else {
            *session_condition = SessionCondition::Ready;
        }
    }
}
