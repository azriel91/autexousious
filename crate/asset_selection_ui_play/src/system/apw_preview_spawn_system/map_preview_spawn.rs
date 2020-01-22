use std::cmp;

use amethyst::{
    ecs::{Entity, Read, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::play::AssetSelection;
use asset_selection_ui_model::play::ApwPreview;
use asset_ui_model::play::AssetSelectionParent;
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::{config::ScaleInit, play::PositionInitParent};
use map_model::loaded::AssetMapBounds;
use map_play::{MapSpawner, MapSpawnerResources};
use parent_model::play::ParentEntity;

use super::PreviewSpawner;

/// Spawns / deletes map preview entities when map selection is switched.
#[derive(Debug, Default, new)]
pub struct MapPreviewSpawn;

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapPreviewSpawnResources<'s> {
    /// `CameraZoomDimensions` resource.
    #[derivative(Debug = "ignore")]
    pub camera_zoom_dimensions: Read<'s, CameraZoomDimensions>,
    /// `AssetMapBounds` resource.
    #[derivative(Debug = "ignore")]
    pub asset_map_bounds: Read<'s, AssetMapBounds>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `PositionInitParent` components.
    #[derivative(Debug = "ignore")]
    pub position_init_parents: WriteStorage<'s, PositionInitParent>,
    /// `ScaleInit` components.
    #[derivative(Debug = "ignore")]
    pub scale_inits: WriteStorage<'s, ScaleInit>,
    /// `MapSpawnerResources`.
    pub map_spawner_resources: MapSpawnerResources<'s>,
}

impl<'s> PreviewSpawner<'s> for MapPreviewSpawn {
    type SystemData = MapPreviewSpawnResources<'s>;

    // Spawns new entities that provide a preview for the asset preview widget.
    fn spawn_preview_entities(
        apw_previews: &mut WriteStorage<'_, ApwPreview>,
        asset_selection_parents: &mut WriteStorage<'_, AssetSelectionParent>,
        map_preview_spawn_resources: &mut MapPreviewSpawnResources,
        ash_entity: Entity,
        apw_main_entity: Option<Entity>,
        asset_selection: AssetSelection,
    ) {
        let MapPreviewSpawnResources {
            camera_zoom_dimensions,
            asset_map_bounds,
            parent_entities,
            position_init_parents,
            scale_inits,
            map_spawner_resources,
        } = map_preview_spawn_resources;

        if let AssetSelection::Id(asset_id) = asset_selection {
            // Scale map to fit within the cell.
            let cell_width = 600.; // TODO: get dimensions.
            let map_width_assumed = asset_map_bounds
                .get(asset_id)
                .map(|map_bounds| {
                    cmp::max(camera_zoom_dimensions.width as u32, map_bounds.width) as f32
                })
                .unwrap_or(camera_zoom_dimensions.width);
            let scale = cell_width / map_width_assumed;

            let map_entities = MapSpawner::spawn(map_spawner_resources, asset_id);
            let parent_entity = ParentEntity::new(ash_entity);
            let spawn_parent_entity = apw_main_entity.unwrap_or(ash_entity);
            map_entities.iter().copied().for_each(|map_entity| {
                apw_previews
                    .insert(map_entity, ApwPreview)
                    .expect("Failed to insert `ApwPreview` component.");
                asset_selection_parents
                    .insert(map_entity, AssetSelectionParent::new(ash_entity))
                    .expect("Failed to insert `AssetSelectionParent` component.");
                parent_entities
                    .insert(map_entity, parent_entity)
                    .expect("Failed to insert `ParentEntity` component.");
                position_init_parents
                    .insert(map_entity, PositionInitParent::new(spawn_parent_entity))
                    .expect("Failed to insert `PositionInitParent` component.");
                scale_inits
                    .insert(map_entity, ScaleInit::new(scale, scale, scale))
                    .expect("Failed to insert `ScaleInit` component.");
            });
        }
    }
}
