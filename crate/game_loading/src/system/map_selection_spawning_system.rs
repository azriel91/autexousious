use amethyst::{
    ecs::{Entities, Entity, Read, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetItemIds, ItemId};
use derivative::Derivative;
use derive_new::new;
use game_model::play::GameEntities;
use log::error;
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
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `GameLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    pub game_loading_status: Write<'s, GameLoadingStatus>,
    /// `MapSelection` resource.
    #[derivative(Debug = "ignore")]
    pub map_selection: Read<'s, MapSelection>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Read<'s, AssetItemIds>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `GameEntities` resource.
    #[derivative(Debug = "ignore")]
    pub game_entities: Write<'s, GameEntities>,
}

impl<'s> System<'s> for MapSelectionSpawningSystem {
    type SystemData = MapSelectionSpawningSystemData<'s>;

    fn run(
        &mut self,
        MapSelectionSpawningSystemData {
            entities,
            mut game_loading_status,
            map_selection,
            asset_id_mappings,
            asset_item_ids,
            mut asset_ids,
            mut item_ids,
            mut game_entities,
        }: Self::SystemData,
    ) {
        if game_loading_status.map_loaded {
            return;
        }

        // TODO: implement Random
        let asset_id = map_selection
            .asset_id()
            .expect("Expected `MapSelection` to contain ID.");
        let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
            panic!(
                "Expected `AssetSlug` to exist for `AssetId`: `{:?}`",
                asset_id
            )
        });
        let map_item_ids = asset_item_ids.get(asset_id).unwrap_or_else(|| {
            let message = format!("Expected `ItemIds` to exist for map `{}`", asset_slug);
            error!("{}", &message);
            panic!("{}", &message);
        });
        let map_entities = map_item_ids
            .iter()
            .copied()
            .map(|item_id| {
                entities
                    .build_entity()
                    .with(asset_id, &mut asset_ids)
                    .with(item_id, &mut item_ids)
                    .build()
            })
            .collect::<Vec<Entity>>();

        game_entities.map_layers = map_entities;
        game_loading_status.map_loaded = true;
    }
}
