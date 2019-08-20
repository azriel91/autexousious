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

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use sequence_model::loaded::WaitSequence;

    use super::SequenceLoadingBundle;

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
