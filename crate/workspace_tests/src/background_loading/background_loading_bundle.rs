#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use background_model::config::BackgroundDefinition;

    use background_loading::BackgroundLoadingBundle;

    #[test]
    fn bundle_build_adds_background_definition_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(BackgroundLoadingBundle::new())
            .with_assertion(|world| {
                world.read_resource::<AssetStorage<BackgroundDefinition>>();
            })
            .run()
    }
}
