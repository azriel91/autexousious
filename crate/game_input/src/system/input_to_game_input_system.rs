use std::convert::TryFrom;

use amethyst::{
    derive::SystemDesc,
    ecs::{Read, System, World, Write},
    input::InputEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{config::ControlBindings, GameInputEvent};

/// Sends `GameInputEvent`s based on a subset of `InputEvent`s.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(InputToGameInputSystemDesc))]
pub struct InputToGameInputSystem {
    /// Reader ID for the `InputEvent` channel.
    #[system_desc(event_channel_reader)]
    input_event_rid: ReaderId<InputEvent<ControlBindings>>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputToGameInputSystemData<'s> {
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Read<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `GameInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub game_input_ec: Write<'s, EventChannel<GameInputEvent>>,
}

impl<'s> System<'s> for InputToGameInputSystem {
    type SystemData = InputToGameInputSystemData<'s>;

    fn run(
        &mut self,
        InputToGameInputSystemData {
            input_ec,
            mut game_input_ec,
        }: Self::SystemData,
    ) {
        input_ec
            .read(&mut self.input_event_rid)
            .filter_map(|input_event| GameInputEvent::try_from(input_event).ok())
            .for_each(|game_input_event| game_input_ec.single_write(game_input_event));
    }
}
