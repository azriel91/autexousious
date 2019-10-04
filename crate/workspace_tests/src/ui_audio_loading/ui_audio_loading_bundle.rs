#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use ui_audio_model::{config::UiSfxPaths, loaded::UiSfxMap, UiAudioLoadingStatus};

    use ui_audio_loading::UiAudioLoadingBundle;

    #[test]
    fn bundle_build_adds_ui_resources() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(UiAudioLoadingBundle::new(PathBuf::default()))
            .with_assertion(|world| {
                // Panics if the Systems weren't added
                world.read_resource::<AssetStorage<UiSfxPaths>>();

                world.read_resource::<UiAudioLoadingStatus>();
                world.read_resource::<UiSfxMap>();
            })
            .run()
    }
}
