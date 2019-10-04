#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use asset_model::loaded::AssetTypeMappings;
    use assets_test::ASSETS_PATH;

    use loading::LoadingBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| {
                world.read_resource::<AssetTypeMappings>();
            })
            .run()
    }
}
