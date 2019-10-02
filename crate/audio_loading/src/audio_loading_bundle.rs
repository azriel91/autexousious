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
