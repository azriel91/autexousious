use amethyst::ecs::Entity;
use asset_model::loaded::AssetId;
use log::error;

use crate::MapSpawnerResources;

/// Spawns a `Map`.
///
/// This is distinct from the `MapSelectionSpawningSystem` so that other systems
/// that spawn game objects may use the same logic, while attaching additional
/// components.
#[derive(Debug)]
pub struct MapSpawner;

impl MapSpawner {
    /// Spawns a `Map` and returns the entities for each layer.
    pub fn spawn(
        MapSpawnerResources {
            entities,
            asset_id_mappings,
            asset_item_ids,
            asset_ids,
            item_ids,
        }: &mut MapSpawnerResources,
        asset_id: AssetId,
    ) -> Vec<Entity> {
        let asset_slug = asset_id_mappings.slug(asset_id).unwrap_or_else(|| {
            panic!(
                "Expected `AssetSlug` to exist for `AssetId`: `{:?}`",
                asset_id
            )
        });
        asset_item_ids
            .get(asset_id)
            .map(|map_item_ids| {
                map_item_ids
                    .iter()
                    .copied()
                    .map(|item_id| {
                        entities
                            .build_entity()
                            .with(asset_id, asset_ids)
                            .with(item_id, item_ids)
                            .build()
                    })
                    .collect::<Vec<Entity>>()
            })
            .unwrap_or_else(|| {
                let message = format!("Expected `ItemIds` to exist for map `{}`", asset_slug);
                error!("{}", &message);
                Vec::new()
            })
    }
}
