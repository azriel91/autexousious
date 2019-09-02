use amethyst::{
    ecs::{Read, System, SystemData, World, Write},
    shrev::{EventChannel, ReaderId},
};
use derive_new::new;
use log::warn;
use stdio_command_model::{StateBarrier, StdinCommandBarrier, StdioCommandEvent};
use typename_derive::TypeName;

/// Updates how `stdin_input` operates based on stdio command events.
#[derive(Debug, Default, TypeName, new)]
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

#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use state_registry::StateId;
    use stdio_command_model::{StateBarrier, StdinCommandBarrier, StdioCommandEvent};
    use typename::TypeName;

    use super::StdioCommandProcessingSystem;

    #[test]
    fn inserts_controller_input_if_non_existent() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(
                StdioCommandProcessingSystem::new(),
                StdioCommandProcessingSystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_effect(|world| {
                world
                    .write_resource::<EventChannel<StdioCommandEvent>>()
                    .single_write(StdioCommandEvent::StateBarrier(StateBarrier {
                        state_id: StateId::GamePlay,
                    })); // kcov-ignore
            })
            .with_assertion(|world| {
                let stdin_command_barrier = world.read_resource::<StdinCommandBarrier>();
                assert_eq!(
                    &StdinCommandBarrier::new(Some(StateId::GamePlay)),
                    &*stdin_command_barrier,
                );
            })
            .run()
    }
}
