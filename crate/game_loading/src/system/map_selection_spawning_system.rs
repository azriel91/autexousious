use amethyst::{
    ecs::{Read, System, World, Write},
    shred::{ResourceId, SystemData},
};
use background_play::{LayerComponentStorages, LayerEntitySpawner, LayerSpawningResources};
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use map_selection_model::MapSelection;
use typename_derive::TypeName;

use crate::GameLoadingStatus;

/// Spawns map entities based on the map selection.
#[derive(Debug, Default, TypeName, new)]
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
    /// `LayerSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub layer_spawning_resources: LayerSpawningResources<'s>,
    /// `LayerComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub layer_component_storages: LayerComponentStorages<'s>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
}

impl<'s> System<'s> for MapSelectionSpawningSystem {
    type SystemData = MapSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionSpawningSystemData {
            mut game_loading_status,
            map_selection,
            layer_spawning_resources,
            mut layer_component_storages,
            mut game_entities,
        }: Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        // TODO: implement Random
        let map_layer_entities = LayerEntitySpawner::spawn_system(
            &layer_spawning_resources,
            &mut layer_component_storages,
            map_selection
                .asset_id()
                .expect("Expected `MapSelection` to contain ID."),
        );

        game_entities.map_layers = map_layer_entities;
        game_loading_status.map_loaded = true;
    }
}
