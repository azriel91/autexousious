use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use map_play::{MapSpawner, MapSpawnerResources};
use map_selection_model::MapSelection;

use crate::GameLoadingStatus;

/// Spawns map entities based on the map selection.
#[derive(Debug, Default, new)]
pub struct MapSelectionSpawningSystem;

/// `MapSelectionSpawningSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionSpawningSystemData<'s> {
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
    /// `MapSpawnerResources`.
    pub map_spawner_resources: MapSpawnerResources<'s>,
}

impl<'s> System<'s> for MapSelectionSpawningSystem {
    type SystemData = MapSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionSpawningSystemData {
            mut game_loading_status,
            map_selection,
            mut game_entities,
            mut map_spawner_resources,
        }: Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        // TODO: implement Random
        let asset_id = map_selection
            .asset_id()
            .expect("Expected map selection to have an `AssetId`.");
        let map_entities = MapSpawner::spawn(&mut map_spawner_resources, asset_id);

        game_entities.map_layers = map_entities;
        game_loading_status.map_loaded = true;
    }
}
