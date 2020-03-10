use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    input::InputEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::config::ControlBindings;
use net_model::play::NetMessageEvent;
use network_session_model::play::SessionStatus;

/// Sends network input to a session server.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetworkInputRequestSystemDesc))]
pub struct NetworkInputRequestSystem {
    /// Reader ID for the `InputEvent<ControlBindings>` channel.
    #[system_desc(event_channel_reader)]
    input_event_rid: ReaderId<InputEvent<ControlBindings>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetworkInputRequestSystemData<'s> {
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Read<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
    /// `NetworkMessageEvent` channel.
    #[derivative(Debug = "ignore")]
    pub net_message_ec: Write<'s, EventChannel<NetMessageEvent>>,
}

impl<'s> System<'s> for NetworkInputRequestSystem {
    type SystemData = NetworkInputRequestSystemData<'s>;

    fn run(
        &mut self,
        NetworkInputRequestSystemData {
            input_ec,
            session_status,
            mut net_message_ec,
        }: Self::SystemData,
    ) {
        let input_events = input_ec.read(&mut self.input_event_rid);

        // Guard against sending input events if the application is not in a session.
        if *session_status == SessionStatus::JoinEstablished
            || *session_status == SessionStatus::HostEstablished
        {
            input_events
                .filter(|ev| match ev {
                    InputEvent::AxisMoved { .. }
                    | InputEvent::ActionPressed(_)
                    | InputEvent::ActionReleased(_) => true,
                    _ => false,
                })
                .cloned()
                .for_each(|ev| {
                    net_message_ec.single_write(NetMessageEvent::InputEvent(ev));
                });
        }
    }
}
