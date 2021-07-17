use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, InteractionsSequence},
};
use derive_new::new;

/// Adds the following systems to the dispatcher.
///
/// * `Processor::<Body>` is added with id `"body_processor"`.
/// * `Processor::<BodySequence>` is added with id `"body_sequence_processor"`.
/// * `Processor::<Interactions>` is added with id `"interactions_processor"`.
/// * `Processor::<InteractionsSequence>` is added with id
///   `"interactions_sequence_processor"`.
#[derive(Debug, new)]
pub struct CollisionLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CollisionLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(Processor::<Body>::new(), "body_processor", &[]); // kcov-ignore
        builder.add(
            Processor::<BodySequence>::new(),
            "body_sequence_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            Processor::<Interactions>::new(),
            "interactions_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            Processor::<InteractionsSequence>::new(),
            "interactions_sequence_processor",
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
