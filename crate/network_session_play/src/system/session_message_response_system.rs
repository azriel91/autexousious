use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::ReaderId,
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::loaded::PlayerControllers;
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::{
    play::{SessionCondition, SessionDeviceJoin, SessionDevices, SessionStatus},
    SessionMessageEvent,
};

/// Records the session code and devices in the world when accepted into a session.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(SessionMessageResponseSystemDesc))]
pub struct SessionMessageResponseSystem {
    /// Reader ID for the `SessionMessageEvent` channel.
    #[system_desc(event_channel_reader)]
    session_message_event_rid: ReaderId<NetData<SessionMessageEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SessionMessageResponseSystemData<'s> {
    /// `SessionMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub session_message_nec: Read<'s, NetEventChannel<SessionMessageEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `SessionCondition` resource.
    #[derivative(Debug = "ignore")]
    pub session_condition: Write<'s, SessionCondition>,
    /// `SessionDevices` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices: Write<'s, SessionDevices>,
    /// `PlayerControllers` resource.
    #[derivative(Debug = "ignore")]
    pub player_controllers: Write<'s, PlayerControllers>,
}

impl<'s> System<'s> for SessionMessageResponseSystem {
    type SystemData = SessionMessageResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionMessageResponseSystemData {
            session_message_nec,
            session_status,
            mut session_condition,
            mut session_devices,
            mut player_controllers,
        }: Self::SystemData,
    ) {
        let session_message_events = session_message_nec.read(&mut self.session_message_event_rid);
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            // Use the last session response even if multiple are received.
            session_message_events.for_each(|ev| {
                let NetData { data, .. } = ev;

                match data {
                    SessionMessageEvent::GameInputTick => {
                        // Have received all `GameInputEvent`s from the session server.
                        *session_condition = SessionCondition::Ready;
                    }
                    SessionMessageEvent::SessionDeviceJoin(session_device_join) => {
                        let SessionDeviceJoin {
                            session_device,
                            player_controllers: player_controllers_received,
                        } = session_device_join;

                        debug!("Session device joined: {:?}", session_device);

                        session_devices.push(session_device.clone());
                        *player_controllers = player_controllers_received.clone();
                    }
                }
            });
        }
    }
}
