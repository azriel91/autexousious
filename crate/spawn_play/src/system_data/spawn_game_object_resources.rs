use amethyst::{
    ecs::{Entities, Read, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::EventChannel,
};
use asset_model::loaded::{AssetId, AssetIdMappings, AssetItemIds, AssetTypeMappings, ItemId};
use character_prefab::{CharacterComponentStorages, CharacterSpawningResources};
use derivative::Derivative;
use energy_prefab::EnergyComponentStorages;
use spawn_model::play::SpawnEvent;

/// `SpawnGameObjectResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectResources<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_id_mappings: Read<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    pub asset_type_mappings: Read<'s, AssetTypeMappings>,
    /// `AssetItemIds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_item_ids: Read<'s, AssetItemIds>,
    /// `AssetId` components.
    #[derivative(Debug = "ignore")]
    pub asset_ids: WriteStorage<'s, AssetId>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `CharacterSpawningResources`.
    #[derivative(Debug = "ignore")]
    pub character_spawning_resources: CharacterSpawningResources<'s>,
    /// `CharacterComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub character_component_storages: CharacterComponentStorages<'s>,
    /// `EnergyComponentStorages`.
    #[derivative(Debug = "ignore")]
    pub energy_component_storages: EnergyComponentStorages<'s>,
    /// `SpawnEvent` channel.
    #[derivative(Debug = "ignore")]
    pub spawn_ec: Write<'s, EventChannel<SpawnEvent>>,
}
