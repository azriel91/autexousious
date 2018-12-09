use amethyst::{assets::AssetStorage, ecs::prelude::*};
use derive_new::new;
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection_model::MapSelection;
use typename_derive::TypeName;

use crate::{GameLoadingStatus, MapLayerComponentStorages, MapLayerEntitySpawner};

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSpawningSystem;

type MapSelectionSpawningSystemData<'s> = (
    Write<'s, GameLoadingStatus>,
    ReadExpect<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    Entities<'s>,
    MapLayerComponentStorages<'s>,
    Write<'s, GameEntities>,
);

impl<'s> System<'s> for MapSelectionSpawningSystem {
    type SystemData = MapSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        (
            mut game_loading_status,
            map_selection,
            loaded_maps,
            entities,
            mut map_component_storages,
            mut game_entities,
        ): Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        let map_spawning_resources = (&*entities, &*loaded_maps);
        let map_layer_entities = MapLayerEntitySpawner::spawn_system(
            &map_spawning_resources,
            &mut map_component_storages,
            map_selection.handle(),
        );

        game_entities.map_layers = map_layer_entities;
        game_loading_status.map_loaded = true;
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, env};

    use amethyst::{assets::ProgressCounter, ecs::prelude::*};
    use amethyst_test::prelude::*;
    use asset_loading::AssetDiscovery;
    use assets_test::{ASSETS_MAP_EMPTY_SLUG, ASSETS_MAP_FADE_SLUG, ASSETS_PATH};
    use game_model::{
        config::AssetSlug,
        loaded::{MapAssets, SlugAndHandle},
        play::GameEntities,
    };
    use loading::AssetLoader;
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use typename::TypeName;

    use super::MapSelectionSpawningSystem;
    use crate::GameLoadingStatus;

    #[test]
    fn returns_if_map_already_loaded() {
        assert!(
            AmethystApplication::render_base("returns_if_map_already_loaded", false)
                .with_bundle(MapLoadingBundle::new())
                .with_setup(load_maps)
                .with_setup(map_selection(ASSETS_MAP_EMPTY_SLUG.clone()))
                .with_setup(|world| {
                    let mut game_loading_status = GameLoadingStatus::new();
                    game_loading_status.map_loaded = true;
                    world.add_resource(game_loading_status);

                    let layer_entity = world.create_entity().build();
                    world.add_resource(GameEntities::new(
                        HashMap::new(),
                        vec![layer_entity.clone()],
                    ));
                    world.add_resource(EffectReturn(layer_entity));
                })
                .with_system_single(
                    MapSelectionSpawningSystem,
                    MapSelectionSpawningSystem::type_name(),
                    &[],
                )
                .with_assertion(|world| {
                    let layer_entity = &world.read_resource::<EffectReturn<Entity>>().0;
                    assert_eq!(
                        layer_entity,
                        world
                            .read_resource::<GameEntities>()
                            .map_layers
                            .iter()
                            .next()
                            .expect("Expected map layers to have an entity.")
                    );
                })
                .run()
                .is_ok()
        );
    }

    #[test]
    fn spawns_map_layers_when_they_havent_been_spawned() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawns_map_layers_when_they_havent_been_spawned",
                false
            )
            .with_bundle(MapLoadingBundle::new())
            .with_setup(load_maps)
            .with_setup(map_selection(ASSETS_MAP_FADE_SLUG.clone()))
            .with_system_single(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            )
            .with_assertion(|world| {
                assert!(!world.read_resource::<GameEntities>().map_layers.is_empty());
                assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
            })
            .run()
            .is_ok()
        );
    }

    #[test]
    fn spawns_map_that_has_no_layers() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawns_map_layers_when_they_havent_been_spawned",
                false
            )
            .with_bundle(MapLoadingBundle::new())
            .with_setup(load_maps)
            .with_setup(map_selection(ASSETS_MAP_EMPTY_SLUG.clone()))
            .with_system_single(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            )
            .with_assertion(|world| {
                assert!(world.read_resource::<GameEntities>().map_layers.is_empty());
                assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
            })
            .run()
            .is_ok()
        );
    }

    fn load_maps(world: &mut World) {
        let asset_index = AssetDiscovery::asset_index(&ASSETS_PATH);

        let mut progress_counter = ProgressCounter::new();
        AssetLoader::load_maps(world, &mut progress_counter, asset_index.maps);
    } // kcov-ignore

    /// Returns a function that adds a `MapSelection` and `MapSelectionStatus::Confirmed`.
    ///
    /// See `application_test_support::SetupFunction`.
    ///
    /// # Parameters
    ///
    /// * `slug`: Asset slug of the map to select.
    fn map_selection(slug: AssetSlug) -> impl Fn(&mut World) {
        move |world| {
            let slug_and_handle = {
                let map_handle = world
                    .read_resource::<MapAssets>()
                    .get(&slug)
                    .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug))
                    .clone();

                SlugAndHandle::from((slug.clone(), map_handle))
            };

            world.add_resource(MapSelection::Id(slug_and_handle));
            world.add_resource(MapSelectionStatus::Confirmed);
        }
    }
}
