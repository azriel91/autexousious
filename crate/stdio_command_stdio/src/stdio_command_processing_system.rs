use amethyst::{
    ecs::{Read, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
};
use derive_new::new;
use log::warn;
use stdio_command_model::{StateBarrier, StdinCommandBarrier, StdioCommandEvent};

/// Updates how `stdin_input` operates based on stdio command events.
#[derive(Debug, Default, new)]
pub struct StdioCommandProcessingSystem {
    /// Reader ID for the `StdioCommandEvent` event channel.
    #[new(default)]
    stdio_command_events_id: Option<ReaderId<StdioCommandEvent>>,
}

type StdioCommandProcessingSystemData<'s> = (
    Read<'s, EventChannel<StdioCommandEvent>>,
    Write<'s, StdinCommandBarrier>,
);

impl<'s> System<'s> for StdioCommandProcessingSystem {
    type SystemData = StdioCommandProcessingSystemData<'s>;

    fn run(&mut self, (input_events, mut stdin_command_barrier): Self::SystemData) {
        let stdio_command_events_id = self
            .stdio_command_events_id
            .as_mut()
            .expect("Expected `stdio_command_events_id` field to be set.");

        input_events
            .read(stdio_command_events_id)
            .for_each(|ev| match ev {
                StdioCommandEvent::StateBarrier(StateBarrier { state_id }) => {
                    if let Some(state_id_existing) = (*stdin_command_barrier).state_id.as_ref() {
                        // kcov-ignore-start
                        warn!(
                            "Existing stdio command barrier exists waiting for state: `{}`.",
                            state_id_existing
                        );
                        // kcov-ignore-end
                    }

                    (*stdin_command_barrier).state_id = Some(*state_id);
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.stdio_command_events_id = Some(
            world
                .fetch_mut::<EventChannel<StdioCommandEvent>>()
                .register_reader(),
        );
    }
}
