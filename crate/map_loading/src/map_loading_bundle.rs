use amethyst::{
    assets::Processor,
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use map_model::loaded::Map;

/// Adds `Processor<Map>` to the `World`.
///
/// This is needed to allow the `loaded::Map` type to be stored in `AssetStorage`.
#[derive(Debug, new)]
pub struct MapLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MapLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(Processor::<Map>::new(), "map_processor", &[]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::assets::{AssetStorage, Loader};
    use amethyst_test_support::prelude::*;
    use map_model::{
        config::{MapBounds, MapDefinition, MapHeader},
        loaded::{Map, MapHandle, Margins},
    };

    use super::MapLoadingBundle;

    #[test]
    fn bundle_build_adds_map_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(MapLoadingBundle::new())
                .with_effect(|world| {
                    let map = test_map();
                    let map_handle: MapHandle = {
                        let loader = world.read_resource::<Loader>();
                        loader.load_from_data(map, (), &world.read_resource())
                    };
                    world.add_resource(EffectReturn(map_handle));
                })
                .with_assertion(|world| {
                    let map_handle = world.read_resource::<EffectReturn<MapHandle>>().0.clone();
                    let store = world.read_resource::<AssetStorage<Map>>();

                    assert_eq!(Some(&test_map()), store.get(&map_handle));
                })
                .run()
                .is_ok()
        );
    }

    fn test_map() -> Map {
        let bounds = MapBounds::new(0, 0, 0, 800, 600, 200);
        let header = MapHeader::new("Test Map".to_string(), bounds);
        let definition = MapDefinition::new(header, Vec::new());
        let margins = Margins::from(definition.header.bounds);
        Map::new(definition, margins)
    }
}
