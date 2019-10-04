#[cfg(test)]
mod test {
    use amethyst::{
        ecs::WorldExt, input::InputBundle, shrev::EventChannel, window::ScreenDimensions, Error,
    };
    use amethyst_test::{AmethystApplication, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH};
    use game_input::GameInputBundle;
    use game_input_model::ControlBindings;
    use stdio_spi::VariantAndTokens;

    use game_input_stdio::GameInputStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_bundle(InputBundle::<ControlBindings>::new())
            .with_bundle(GameInputBundle::new())
            .with_bundle(GameInputStdioBundle::new())
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
            })
            // kcov-ignore-end
            .run()
    }
}
