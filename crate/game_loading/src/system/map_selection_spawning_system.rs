use amethyst::{assets::AssetStorage, ecs::prelude::*};
use game_model::play::GameEntities;
use map_model::loaded::Map;
use map_selection::MapSelection;

use MapLayerComponentStorages;
use MapLayerEntitySpawner;

/// Spawns character entities based on the character selection.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct MapSelectionSpawningSystem;

type MapSelectionSpawningSystemData<'s> = (
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
            map_selection,
            loaded_maps,
            entities,
            mut map_component_storages,
            mut game_entities,
        ): Self::SystemData,
){
        if !game_entities.map_layers.is_empty() {
            // Already populated
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
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::path::Path;

    use amethyst::{assets::ProgressCounter, ecs::prelude::*};
    use amethyst_test_support::prelude::*;
    use application::resource::dir::ASSETS;
    use game_model::{config::index_configuration, play::GameEntities};
    use loading::AssetLoader;
    use map_loading::MapLoadingBundle;
    use map_model::loaded::MapHandle;
    use map_selection::{MapSelection, MapSelectionStatus};
    use typename::TypeName;

    use super::MapSelectionSpawningSystem;

    #[test]
    fn returns_if_map_already_populated() {
        assert!(
            AmethystApplication::render_base("returns_if_map_already_populated", false)
                .with_bundle(MapLoadingBundle::new())
                .with_system(
                    MapSelectionSpawningSystem,
                    MapSelectionSpawningSystem::type_name(),
                    &[],
                ).with_setup(|world| {
                    let layer_entity = world.create_entity().build();
                    world.add_resource(GameEntities::new(
                        HashMap::new(),
                        vec![layer_entity.clone()],
                    ));
                    world.add_resource(EffectReturn(layer_entity));
                }).with_assertion(|world| {
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
            .with_system(
                MapSelectionSpawningSystem,
                MapSelectionSpawningSystem::type_name(),
                &[],
            ).with_setup(|world| {
                // We need to put all of the setup in one function because all of this needs to run
                // before the system runs.
                let assets_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(ASSETS);
                let configuration_index = index_configuration(&assets_dir);

                let mut progress_counter = ProgressCounter::new();
                AssetLoader::load_maps(world, &mut progress_counter, &configuration_index);

                let first_map_handle = world
                    .read_resource::<Vec<MapHandle>>()
                    // TODO: <https://gitlab.com/azriel91/autexousious/issues/57>
                    .get(1) // assets/test/map/fade
                    .expect("Expected at least one map to be loaded.")
                    .clone();
                let map_selection =
                    MapSelection::new(MapSelectionStatus::Confirmed, Some(first_map_handle));

                world.add_resource(map_selection);
            }).with_assertion(|world| {
                assert!(!world.read_resource::<GameEntities>().map_layers.is_empty());
            }).run()
            .is_ok()
        );
    }
}
