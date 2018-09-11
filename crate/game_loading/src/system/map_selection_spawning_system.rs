use amethyst::{assets::AssetStorage, ecs::prelude::*};
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection::MapSelection;

use GameLoadingStatus;
use MapLayerComponentStorages;
use MapLayerEntitySpawner;

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSpawningSystem;

type MapSelectionSpawningSystemData<'s> = (
    Write<'s, GameLoadingStatus>,
    Read<'s, MapSelection>,
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

        // Read map to determine bounds where the characters can be spawned.
        let map_handle = map_selection
            .map_handle
            .as_ref()
            .expect("Expected map to be selected.");

        let map_spawning_resources = (&*entities, &*loaded_maps);

        let map_layer_entities = MapLayerEntitySpawner::spawn_system(
            &map_spawning_resources,
            &mut map_component_storages,
            map_handle,
        );

        game_entities.map_layers = map_layer_entities;
        game_loading_status.map_loaded = true;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use amethyst::{assets::ProgressCounter, ecs::prelude::*};
    use amethyst_test_support::prelude::*;
    use asset_loading::AssetDiscovery;
    use assets_test::{ASSETS_MAP_EMPTY_SLUG, ASSETS_MAP_FADE_SLUG, ASSETS_PATH};
    use game_model::{loaded::MapAssets, play::GameEntities};
    use loading::AssetLoader;
    use map_loading::MapLoadingBundle;
    use map_selection::{MapSelection, MapSelectionStatus};
    use typename::TypeName;

    use super::MapSelectionSpawningSystem;
    use GameLoadingStatus;

    #[test]
    fn returns_if_map_already_loaded() {
        assert!(
            AmethystApplication::render_base("returns_if_map_already_loaded", false)
                .with_bundle(MapLoadingBundle::new())
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
                }).with_system_single(
                    MapSelectionSpawningSystem,
                    MapSelectionSpawningSystem::type_name(),
                    &[],
                ).with_assertion(|world| {
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
                }).run()
                .is_ok()
        );
    }

    // kcov-ignore-start
    #[test]
    #[ignore] // We can't test for panics because it poisons the test support Mutex
    #[should_panic]
    fn panics_when_map_selection_resource_not_present() {
        AmethystApplication::render_base("panics_when_map_selection_resource_not_present", false)
            .with_bundle(MapLoadingBundle::new())
            .with_system(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            ).with_assertion(|_| {})
            .run()
            .ok();
    }
    // kcov-ignore-end

    #[test]
    fn spawns_map_layers_when_they_havent_been_spawned() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base(
                "spawns_map_layers_when_they_havent_been_spawned",
                false
            ).with_bundle(MapLoadingBundle::new())
            .with_setup(|world| {
                let asset_index = AssetDiscovery::asset_index(&ASSETS_PATH);

                let mut progress_counter = ProgressCounter::new();
                AssetLoader::load_maps(world, &mut progress_counter, asset_index.maps);
            }).with_setup(|world| {
                let fade_map_handle = world
                    .read_resource::<MapAssets>()
                    .get(&*ASSETS_MAP_FADE_SLUG)
                    .expect(&format!(
                        "Expected `{}` map to be loaded.",
                        *ASSETS_MAP_FADE_SLUG
                    )).clone();

                world.add_resource(MapSelection::new(Some(fade_map_handle)));
                world.add_resource(MapSelectionStatus::Confirmed);
            }).with_system_single(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            ).with_assertion(|world| {
                assert!(!world.read_resource::<GameEntities>().map_layers.is_empty());
                assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
            }).run()
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
            ).with_bundle(MapLoadingBundle::new())
            .with_setup(|world| {
                let asset_index = AssetDiscovery::asset_index(&ASSETS_PATH);

                let mut progress_counter = ProgressCounter::new();
                AssetLoader::load_maps(world, &mut progress_counter, asset_index.maps);
            }).with_setup(|world| {
                let empty_map_handle = world
                    .read_resource::<MapAssets>()
                    .get(&*ASSETS_MAP_EMPTY_SLUG)
                    .expect(&format!(
                        "Expected `{}` map to be loaded.",
                        *ASSETS_MAP_EMPTY_SLUG
                    )).clone();

                world.add_resource(MapSelection::new(Some(empty_map_handle)));
                world.add_resource(MapSelectionStatus::Confirmed);
            }).with_system_single(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            ).with_assertion(|world| {
                assert!(world.read_resource::<GameEntities>().map_layers.is_empty());
                assert!(world.read_resource::<GameLoadingStatus>().map_loaded);
            }).run()
            .is_ok()
        );
    }
}
