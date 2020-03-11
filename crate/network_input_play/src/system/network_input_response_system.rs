use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::GameInputEvent;
use log::debug;
use net_model::play::{NetData, NetEventChannel};
use network_session_model::play::SessionStatus;

/// Writes received `GameInputEvent`s from the net channel to the regular event channel.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(NetworkInputResponseSystemDesc))]
pub struct NetworkInputResponseSystem {
    /// Reader ID for the `GameInputEvent` channel.
    #[system_desc(event_channel_reader)]
    game_input_event_rid: ReaderId<NetData<GameInputEvent>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct NetworkInputResponseSystemData<'s> {
    /// Net `GameInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_input_nec: Read<'s, NetEventChannel<GameInputEvent>>,
    /// `GameInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_input_ec: Write<'s, EventChannel<GameInputEvent>>,
    /// `SessionStatus` resource.
    #[derivative(Debug = "ignore")]
    pub session_status: Read<'s, SessionStatus>,
}

impl<'s> System<'s> for NetworkInputResponseSystem {
    type SystemData = NetworkInputResponseSystemData<'s>;

    fn run(
        &mut self,
        NetworkInputResponseSystemData {
            game_input_nec,
            mut game_input_ec,
            session_status,
        }: Self::SystemData,
    ) {
        let game_input_events = game_input_nec.read(&mut self.game_input_event_rid);
        let session_status = &*session_status;

        if session_status == &SessionStatus::JoinEstablished
            || session_status == &SessionStatus::HostEstablished
        {
            game_input_events.for_each(|ev| {
                let NetData {
                    data: game_input_event,
                    ..
                } = ev;

                debug!(
                    "`NetData<GameInputEvent>` received: {:?}.",
                    game_input_event
                );

                game_input_ec.single_write(*game_input_event);
            });
        }
    }
}
