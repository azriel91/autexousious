#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use spawn_model::loaded::{Spawns, SpawnsSequence};

    use spawn_loading::SpawnLoadingBundle;

    #[test]
    fn bundle_build_adds_body_and_spawns_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpawnLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<Spawns>>();
                world.read_resource::<AssetStorage<SpawnsSequence>>();
            })
            .run()
    }
}
