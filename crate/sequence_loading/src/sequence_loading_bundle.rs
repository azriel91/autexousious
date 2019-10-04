use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use sequence_model::loaded::WaitSequence;

/// Adds the following systems to the dispatcher:
///
/// * `Processor::<WaitSequence>`
#[derive(Debug, new)]
pub struct SequenceLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SequenceLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<WaitSequence>::new(),
            "wait_sequence_processor",
            &[],
        );
        Ok(())
    }
}
