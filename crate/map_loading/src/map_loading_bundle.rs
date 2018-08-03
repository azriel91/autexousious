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
    use std::path::Path;

    use amethyst::assets::AssetStorage;
    use amethyst_test_support::prelude::*;
    use application::resource::dir::assets_dir;
    use map_model::loaded::{Map, MapHandle};

    use super::MapLoadingBundle;
    use MapLoader;

    #[test]
    fn bundle_build_adds_map_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_map_assets", false)
                .with_bundle(MapLoadingBundle)
                .with_effect(|world| {
                    let mut map_path = assets_dir(Some(development_base_dirs!()))
                        .expect("Expected to find `assets` directory in crate root.");
                    map_path.extend(Path::new("test/map/fade").iter());

                    let map_handle = MapLoader::load(world, &map_path).expect("Failed to load map");

                    world.add_resource(EffectReturn(map_handle));
                })
                .with_assertion(|world| {
                    let map_handle = world.read_resource::<EffectReturn<MapHandle>>().0.clone();
                    let map_store = world.read_resource::<AssetStorage<Map>>();
                    let map = map_store
                        .get(&map_handle)
                        .expect("Expected map to be loaded");

                    // See fade/map.toml
                    assert_eq!(
                        2,
                        map.animation_handles
                            .as_ref()
                            .expect("Expected test/map/fade map to contain animations.")
                            .len()
                    );
                })
                .run()
                .is_ok()
        );
    }
}
