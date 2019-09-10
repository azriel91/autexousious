use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use audio_model::loaded::SourceSequence;
use derive_new::new;

/// Adds the following systems to the dispatcher.
///
/// * `Processor::<SourceSequence>` is added with id `"source_sequence_processor"`.
#[derive(Debug, new)]
pub struct AudioLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for AudioLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        // Processor::<Source> is added by the `AudioBundle` from `amethyst_audio`.
        builder.add(
            Processor::<SourceSequence>::new(),
            "source_sequence_processor",
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use audio_model::loaded::SourceSequence;

    use super::AudioLoadingBundle;

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
