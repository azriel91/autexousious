use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use map_model::{config::MapDefinition, loaded::Map};

/// Adds the following `System`s to the `World`:
///
/// * `Processor<Map>`
/// * `Processor<MapDefinition>`
#[derive(Debug, new)]
pub struct MapLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MapLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<MapDefinition>::new(),
            "map_definition_processor",
            &[],
        );
        builder.add(
            Processor::<Map>::new(),
            "map_processor",
            &["map_definition_processor"],
        );
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use map_model::{config::MapDefinition, loaded::Map};

    use super::MapLoadingBundle;

    #[test]
    fn bundle_build_adds_map_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(MapLoadingBundle::new())
            .with_assertion(|world| {
                world.read_resource::<AssetStorage<Map>>();
                world.read_resource::<AssetStorage<MapDefinition>>();
            })
            .run()
    }
}
