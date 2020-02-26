use std::cmp;

use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, Read, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::AssetId, ItemComponent};
use camera_model::play::CameraZoomDimensions;
use derivative::Derivative;
use derive_new::new;
use kinematic_model::{config::ScaleInit, play::PositionInitParent};
use map_model::loaded::AssetMapBounds;
use map_play::{MapSpawner, MapSpawnerResources};
use parent_model::play::ParentEntity;
use ui_model_spi::config::Dimensions;

/// Display cell for a map asset.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetDisplayCellMap {
    /// ID of the asset to display.
    pub asset_id: AssetId,
    /// Size of the cell.
    ///
    /// This is used to position the spawned asset.
    pub cell_size: Dimensions,
}

impl AsRef<AssetId> for AssetDisplayCellMap {
    fn as_ref(&self) -> &AssetId {
        &self.asset_id
    }
}

impl AssetDisplayCellMap {
    fn spawn_map(
        &self,
        AssetDisplayCellMapSystemData {
            camera_zoom_dimensions,
            asset_map_bounds,
            parent_entities,
            position_init_parents,
            scale_inits,
            map_spawner_resources,
            ..
        }: &mut AssetDisplayCellMapSystemData,
        entity: Entity,
    ) {
        // Scale map to fit within the cell.
        let cell_width = self.cell_size.w as f32;
        let map_width_assumed = asset_map_bounds
            .get(self.asset_id)
            .map(|map_bounds| {
                cmp::max(camera_zoom_dimensions.width as u32, map_bounds.width) as f32
            })
            .unwrap_or(camera_zoom_dimensions.width);
        let scale = cell_width / map_width_assumed;

        let map_entities = MapSpawner::spawn(map_spawner_resources, self.asset_id);
        let parent_entity = ParentEntity::new(entity);
        map_entities.iter().copied().for_each(|map_entity| {
            parent_entities
                .insert(map_entity, parent_entity)
                .expect("Failed to insert `ParentEntity` component.");
            position_init_parents
                .insert(map_entity, PositionInitParent::new(entity))
                .expect("Failed to insert `PositionInitParent` component.");
            scale_inits
                .insert(map_entity, ScaleInit::new(scale, scale, scale))
                .expect("Failed to insert `ScaleInit` component.");
        });
    }
}

/// `AssetDisplayCellMapSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDisplayCellMapSystemData<'s> {
    /// `AssetDisplayCellMap` components.
    #[derivative(Debug = "ignore")]
    pub asset_display_cells: WriteStorage<'s, AssetDisplayCellMap>,
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

impl<'s> ItemComponent<'s> for AssetDisplayCellMap {
    type SystemData = AssetDisplayCellMapSystemData<'s>;

    fn augment(&self, mut asset_display_cell_system_data: &mut Self::SystemData, entity: Entity) {
        if !asset_display_cell_system_data
            .asset_display_cells
            .contains(entity)
        {
            asset_display_cell_system_data
                .asset_display_cells
                .insert(entity, *self)
                .expect("Failed to insert `AssetDisplayCellMap` component.");

            self.spawn_map(&mut asset_display_cell_system_data, entity);
        }
    }
}
