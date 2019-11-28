use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Entity, Read, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use asset_model::{
    config::AssetType,
    loaded::{AssetId, AssetIdMappings, AssetItemIds, AssetTypeMappings, ItemId},
};
use character_prefab::{
    CharacterComponentStorages, CharacterEntityAugmenter, CharacterSpawningResources,
};
use derivative::Derivative;
use derive_new::new;
use energy_prefab::{EnergyComponentStorages, EnergyEntityAugmenter};
use log::error;
use object_type::ObjectType;
use sequence_model::play::SequenceUpdateEvent;
use spawn_model::{
    loaded::{Spawns, SpawnsHandle},
    play::SpawnEvent,
};
use typename_derive::TypeName;

/// Spawns `GameObject`s. Currently only supports `Energy` objects.
#[derive(Debug, Default, TypeName, new)]
pub struct SpawnGameObjectSystem {
    /// Reader ID for the `SequenceUpdateEvent` event channel.
    #[new(default)]
    reader_id: Option<ReaderId<SequenceUpdateEvent>>,
}

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

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpawnGameObjectSystemData<'s> {
    /// `SpawnGameObjectResources`.
    pub spawn_game_object_resources: SpawnGameObjectResources<'s>,
    /// Event channel for `SequenceUpdateEvent`s.
    #[derivative(Debug = "ignore")]
    pub sequence_update_ec: Read<'s, EventChannel<SequenceUpdateEvent>>,
    /// `SpawnsHandle` components.
    #[derivative(Debug = "ignore")]
    pub spawns_handles: ReadStorage<'s, SpawnsHandle>,
    /// `Spawns` assets.
    #[derivative(Debug = "ignore")]
    pub spawns_assets: Read<'s, AssetStorage<Spawns>>,
}

impl SpawnGameObjectSystem {
    /// Creates an entity for each `Spawn` and attaches its prefab handle.
    fn spawn_game_objects(
        SpawnGameObjectResources {
            entities,
            asset_id_mappings,
            asset_type_mappings,
            asset_item_ids,
            asset_ids,
            item_ids,
            character_spawning_resources,
            character_component_storages,
            energy_component_storages,
            spawn_ec,
        }: &mut SpawnGameObjectResources<'_>,
        spawns: &Spawns,
        entity_parent: Entity,
    ) {
        spawns.iter().for_each(|spawn| {
            let asset_id = spawn.object;
            let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
                panic!(
                    "Expected `AssetSlug` to exist for `AssetId`: {:?}",
                    asset_id
                )
            });
            let item_ids_character = asset_item_ids.get(asset_id).unwrap_or_else(|| {
                panic!("Expected `ItemIds` to exist for asset: `{}`", asset_slug);
            });
            let item_id = item_ids_character.first().copied().unwrap_or_else(|| {
                panic!("Expected `ItemId` to exist for asset: `{}`", asset_slug)
            });

            let asset_type = asset_type_mappings.get(asset_id).unwrap_or_else(|| {
                panic!(
                    "`AssetType` not found for `{:?}`, slug: `{}`.",
                    asset_id, asset_slug
                )
            });
            let entity_spawned = entities.create();

            asset_ids
                .insert(entity_spawned, asset_id)
                .expect("Failed to insert `AssetId`.");
            item_ids
                .insert(entity_spawned, item_id)
                .expect("Failed to insert `ItemId`.");

            match asset_type {
                AssetType::Object(ObjectType::Character) => {
                    CharacterEntityAugmenter::augment(
                        character_spawning_resources,
                        character_component_storages,
                        asset_id,
                        entity_spawned,
                    );
                }
                AssetType::Object(ObjectType::Energy) => {
                    EnergyEntityAugmenter::augment(entity_spawned, energy_component_storages);
                }
                _ => {
                    let asset_slug = asset_id_mappings
                        .slug(asset_id)
                        .unwrap_or_else(|| panic!("`AssetSlug` not found for `{:?}`.", asset_id));
                    error!(
                        "Spawning of asset type `{:?}` (`{}`) is not supported.",
                        asset_type, asset_slug
                    );
                }
            }

            let spawn_event =
                SpawnEvent::new(spawn.clone(), entity_parent, entity_spawned, asset_id);
            spawn_ec.single_write(spawn_event);
        });
    }
}

impl<'s> System<'s> for SpawnGameObjectSystem {
    type SystemData = SpawnGameObjectSystemData<'s>;

    fn run(
        &mut self,
        SpawnGameObjectSystemData {
            mut spawn_game_object_resources,
            sequence_update_ec,
            spawns_handles,
            spawns_assets,
        }: Self::SystemData,
    ) {
        sequence_update_ec
            .read(
                self.reader_id
                    .as_mut()
                    .expect("Expected reader ID to exist for FrameComponentUpdateSystem."),
            )
            .filter(|ev| {
                if let SequenceUpdateEvent::SequenceBegin { .. }
                | SequenceUpdateEvent::FrameBegin { .. } = ev
                {
                    true
                } else {
                    false
                }
            })
            .for_each(|ev| {
                let entity_parent = ev.entity();
                let spawns_handle = spawns_handles.get(entity_parent);

                // Some entities will have sequence update events, but not a spawns handle
                // component.
                if let Some(spawns_handle) = spawns_handle {
                    let spawns = spawns_assets
                        .get(spawns_handle)
                        .expect("Expected `Spawns` to be loaded.");

                    Self::spawn_game_objects(
                        &mut spawn_game_object_resources,
                        spawns,
                        entity_parent,
                    );
                }
            });
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(
            world
                .fetch_mut::<EventChannel<SequenceUpdateEvent>>()
                .register_reader(),
        );
    }
}
