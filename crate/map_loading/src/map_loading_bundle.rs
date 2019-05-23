use amethyst::{assets::Processor, core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use map_model::{config::MapDefinition, loaded::Map};

/// Adds the following `System`s to the `World`:
///
/// * `Processor<Map>`
/// * `Processor<MapDefinition>`
#[derive(Debug, new)]
pub struct MapLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MapLoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
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
    use amethyst::assets::AssetStorage;
    use amethyst_test::{AmethystApplication, EffectReturn, RenderBaseAppExt};
    use assets_test::{ASSETS_MAP_FADE_PATH, ASSETS_MAP_FADE_SLUG};
    use map_model::loaded::{Map, MapHandle};
    use sequence_loading::SequenceLoadingBundle;

    use super::MapLoadingBundle;
    use crate::MapLoader;

    #[test]
    fn bundle_build_adds_map_processor() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base()
                .with_bundle(SequenceLoadingBundle::new())
                .with_bundle(MapLoadingBundle::new())
                .with_effect(|world| {
                    let map_handle =
                        MapLoader::load(world, &ASSETS_MAP_FADE_PATH).expect("Failed to load map");

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
                        map.component_sequences_handles
                            .as_ref()
                            .expect(&format!(
                                "Expected '{}' map to contain component_sequences_handles.",
                                *ASSETS_MAP_FADE_SLUG
                            ))
                            .len()
                    );
                })
                .run()
                .is_ok()
        );
    }
}
