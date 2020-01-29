use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_model::{config::AssetType, loaded::AssetTypeMappings};
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use log::warn;
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
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
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
            asset_type_mappings,
            mut map_spawner_resources,
        }: Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        // TODO: implement Random
        let asset_id = map_selection.asset_id().unwrap_or_else(|| {
            // TODO: Fix `MapSelectionSystem`, `MapSelectionSpawningSystem`, and
            // `CharacterAugmentRectifySystem`
            warn!("Expected map selection to have an `AssetId`.");
            asset_type_mappings
                .iter_ids(&AssetType::Map)
                .next()
                .copied()
                .expect("Expected at least one map to be loaded.")
        });
        let map_entities = MapSpawner::spawn(&mut map_spawner_resources, asset_id);

        game_entities.map_layers = map_entities;
        game_loading_status.map_loaded = true;
    }
}
