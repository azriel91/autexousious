use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use spawn_model::loaded::{Spawns, SpawnsSequence};

/// Adds the following systems to the dispatcher.
///
/// * `Processor::<Spawns>` is added with id `"spawns_processor"`.
/// * `Processor::<SpawnsSequence>` is added with id
///   `"spawns_sequence_processor"`.
#[derive(Debug, new)]
pub struct SpawnLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpawnLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(Processor::<Spawns>::new(), "spawns_processor", &[]); // kcov-ignore
        builder.add(
            Processor::<SpawnsSequence>::new(),
            "spawns_sequence_processor",
            &[],
        ); // kcov-ignore
        Ok(())
    }
}
