#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use state_registry::StateId;
    use stdio_command_model::{StateBarrier, StdinCommandBarrier, StdioCommandEvent};
    use typename::TypeName;

    use stdio_command_stdio::StdioCommandProcessingSystem;

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
