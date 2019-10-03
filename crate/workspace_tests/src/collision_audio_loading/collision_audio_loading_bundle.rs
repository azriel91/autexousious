#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use collision_audio_model::{
        config::CollisionSfxPaths, loaded::CollisionSfxMap, CollisionAudioLoadingStatus,
    };

    use collision_audio_loading::CollisionAudioLoadingBundle;

    #[test]
    fn bundle_build_adds_collision_resources() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CollisionAudioLoadingBundle::new(PathBuf::default()))
            .with_assertion(|world| {
                // Panics if the Systems weren't added
                world.read_resource::<AssetStorage<CollisionSfxPaths>>();

                world.read_resource::<CollisionAudioLoadingStatus>();
                world.read_resource::<CollisionSfxMap>();
            })
            .run()
    }
}
