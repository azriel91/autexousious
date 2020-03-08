use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::ReaderId,
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::{
    play::{SessionDeviceJoin, SessionDevices, SessionStatus},
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
    /// `SessionDevices` resource.
    #[derivative(Debug = "ignore")]
    pub session_devices: Write<'s, SessionDevices>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
}

impl<'s> System<'s> for SessionMessageResponseSystem {
    type SystemData = SessionMessageResponseSystemData<'s>;

    fn run(
        &mut self,
        SessionMessageResponseSystemData {
            session_message_nec,
            mut session_devices,
            session_status,
        }: Self::SystemData,
    ) {
        let session_message_events = session_message_nec.read(&mut self.session_message_event_rid);
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            // Use the last session response even if multiple are received.
            session_message_events.for_each(|ev| match ev {
                NetData {
                    data: SessionMessageEvent::SessionDeviceJoin(session_device_join),
                    ..
                } => {
                    let SessionDeviceJoin { session_device } = session_device_join;

                    debug!("Session device joined: {:?}", session_device);

                    session_devices.push(session_device.clone());
                }
            });
        }
    }
}
