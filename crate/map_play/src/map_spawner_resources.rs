use amethyst::{
    ecs::{Entities, Read, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetItemIds, ItemId};
use derivative::Derivative;

/// `MapSpawnerResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSpawnerResources<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
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
}
