#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use sequence_model::loaded::WaitSequence;

    use sequence_loading::SequenceLoadingBundle;

    #[test]
    fn bundle_build_adds_sequence_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SequenceLoadingBundle)
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<WaitSequence>>();
            })
            .run()
    }
}
