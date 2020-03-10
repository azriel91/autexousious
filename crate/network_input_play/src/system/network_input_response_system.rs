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
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::play::SessionStatus;

/// Writes received `InputEvent<ControlBindings>`s from the net channel to the regular event channel.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetworkInputResponseSystemDesc))]
pub struct NetworkInputResponseSystem {
    /// Reader ID for the `InputEvent<ControlBindings>` channel.
    #[system_desc(event_channel_reader)]
    network_input_event_rid: ReaderId<NetData<InputEvent<ControlBindings>>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetworkInputResponseSystemData<'s> {
    /// Net `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub network_input_nec: Read<'s, NetEventChannel<InputEvent<ControlBindings>>>,
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Write<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
}

impl<'s> System<'s> for NetworkInputResponseSystem {
    type SystemData = NetworkInputResponseSystemData<'s>;

    fn run(
        &mut self,
        NetworkInputResponseSystemData {
            network_input_nec,
            mut input_ec,
            session_status,
        }: Self::SystemData,
    ) {
        let network_input_events = network_input_nec.read(&mut self.network_input_event_rid);
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            network_input_events.for_each(|ev| {
                let NetData {
                    data: input_event, ..
                } = ev;

                debug!("`NetData<InputEvent>` received.");

                input_ec.single_write(input_event.clone());
            });
        }
    }
}
