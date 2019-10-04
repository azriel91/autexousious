#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use application_input::ApplicationEvent;
    use state_registry::StateId;

    use stdio_input::StdioInputBundle;

    #[test]
    fn bundle_should_add_stdin_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(StdioInputBundle)
            .with_resource(StateId::Loading)
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<ApplicationEvent>>();
            })
            // kcov-ignore-end
            .run()
    }
}
