#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use audio_model::loaded::SourceSequence;

    use audio_loading::AudioLoadingBundle;

    #[test]
    fn bundle_build_adds_source_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(AudioLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<SourceSequence>>();
            })
            .run()
    }
}
