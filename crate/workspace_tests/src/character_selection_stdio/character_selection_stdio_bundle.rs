#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use asset_model::loaded::AssetIdMappings;
    use stdio_spi::VariantAndTokens;

    use character_selection_stdio::CharacterSelectionStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CharacterSelectionStdioBundle::new())
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
                world.read_resource::<AssetIdMappings>();
            })
            // kcov-ignore-end
            .run()
    }
}
