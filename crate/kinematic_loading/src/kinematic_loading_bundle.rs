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
