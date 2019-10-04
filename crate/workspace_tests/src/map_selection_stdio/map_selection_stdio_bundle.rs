#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use asset_model::loaded::{AssetIdMappings, AssetTypeMappings};
    use stdio_spi::VariantAndTokens;

    use map_selection_stdio::MapSelectionStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(MapSelectionStdioBundle::new())
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
                world.read_resource::<AssetIdMappings>();
                world.read_resource::<AssetTypeMappings>();
            })
            .run()
    }
}
