use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use spawn_model::{config::Spawns, loaded::SpawnsSequence};

/// Adds the following systems to the dispatcher.
///
/// * `Processor::<Spawns>` is added with id `"spawns_processor"`.
/// * `Processor::<SpawnsSequence>` is added with id `"spawns_sequence_processor"`.
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

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use spawn_model::{config::Spawns, loaded::SpawnsSequence};

    use super::SpawnLoadingBundle;

    #[test]
    fn bundle_build_adds_body_and_spawns_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpawnLoadingBundle::new())
            .with_assertion(|world| {
                // Next line will panic if the Processors aren't added
                world.read_resource::<AssetStorage<Spawns>>();
                world.read_resource::<AssetStorage<SpawnsSequence>>();
            })
            .run()
    }
}
