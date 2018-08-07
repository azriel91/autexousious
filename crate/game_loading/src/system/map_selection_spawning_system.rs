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
