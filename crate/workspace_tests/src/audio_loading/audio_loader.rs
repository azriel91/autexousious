#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst::{
        assets::{AssetStorage, Handle, Loader, Processor, ProgressCounter},
        audio::Source,
        ecs::WorldExt,
        Error,
    };
    use amethyst_test::{AmethystApplication, WaitForLoad};

    use audio_loading::AudioLoader;

    #[test]
    fn loads_wav_files() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_effect(|world| {
                let mut progress_counter = ProgressCounter::new();

                let source_handle = {
                    let loader = world.read_resource::<Loader>();
                    let source_assets = world.read_resource::<AssetStorage<Source>>();
                    let path = Path::new("test/sfx/empty.wav");

                    AudioLoader::load(&loader, &source_assets, &mut progress_counter, path)
                };

                world.insert(source_handle);
                world.insert(progress_counter);
            })
            .with_state(WaitForLoad::new)
            .with_assertion(|world| {
                let source_handle = world.read_resource::<Handle<Source>>().clone();
                let source_assets = world.read_resource::<AssetStorage<Source>>();

                assert!(source_assets.get(&source_handle).is_some());
            })
            .run()
    }
}
