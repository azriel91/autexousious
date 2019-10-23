#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use map_model::config::MapDefinition;

    use map_loading::MapLoadingBundle;

    #[test]
    fn bundle_build_adds_map_definition_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(MapLoadingBundle::new())
            .with_assertion(|world| {
                world.read_resource::<AssetStorage<MapDefinition>>();
            })
            .run()
    }
}
