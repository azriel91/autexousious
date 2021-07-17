use amethyst::ecs::Entity;
use asset_model::config::AssetType;
use character_prefab::CharacterEntityAugmenter;
use energy_prefab::EnergyEntityAugmenter;
use log::{debug, error};
use object_type::ObjectType;
use spawn_model::{loaded::Spawn, play::SpawnEvent};

use crate::SpawnGameObjectResources;

/// Spawns a `GameObject` and sends a `SpawnEvent`.
///
/// This is distinct from the `SpawnGameObjectSystem` so that other systems that
/// spawn game objects may use the same logic, while attaching additional
/// components.
#[derive(Debug)]
pub struct GameObjectSpawner;

impl GameObjectSpawner {
    /// Spawns a `GameObject` and sends a `SpawnEvent`.
    pub fn spawn(
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
        entity_parent: Entity,
        spawn: &Spawn,
    ) -> Entity {
        let asset_id = spawn.object;
        let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
            panic!(
                "Expected `AssetSlug` to exist for `AssetId`: {:?}",
                asset_id
            )
        });

        debug!("Spawning entity for asset: `{}`.", asset_slug);

        let item_ids_character = asset_item_ids.get(asset_id).unwrap_or_else(|| {
            panic!("Expected `ItemIds` to exist for asset: `{}`", asset_slug);
        });
        let item_id = item_ids_character
            .first()
            .copied()
            .unwrap_or_else(|| panic!("Expected `ItemId` to exist for asset: `{}`", asset_slug));

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

        let spawn_event = SpawnEvent::new(spawn.clone(), entity_parent, entity_spawned, asset_id);
        spawn_ec.single_write(spawn_event);

        entity_spawned
    }
}
