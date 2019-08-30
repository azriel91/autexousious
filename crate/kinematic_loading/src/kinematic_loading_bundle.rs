use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use kinematic_model::loaded::ObjectAccelerationSequence;

/// Adds the following systems to the dispatcher.
///
/// * `Processor::<ObjectAccelerationSequence>` is added with id `"object_acceleration_sequence_processor"`.
#[derive(Debug, new)]
pub struct KinematicLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for KinematicLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<ObjectAccelerationSequence>::new(),
            "object_acceleration_sequence_processor",
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use kinematic_model::loaded::ObjectAccelerationSequence;

    use super::KinematicLoadingBundle;

    #[test]
    fn bundle_build_adds_object_acceleration_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(KinematicLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<ObjectAccelerationSequence>>();
            })
            .run()
    }
}
