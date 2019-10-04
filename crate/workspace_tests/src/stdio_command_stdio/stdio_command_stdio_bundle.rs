#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use state_registry::StateId;
    use stdio_input::StdioInputBundle;
    use stdio_spi::VariantAndTokens;

    use stdio_command_stdio::StdioCommandStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(StdioInputBundle::new())
            .with_bundle(StdioCommandStdioBundle::new())
            .with_resource(StateId::Loading)
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
            })
            // kcov-ignore-end
            .run()
    }
}
