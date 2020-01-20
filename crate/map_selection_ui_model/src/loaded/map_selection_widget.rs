use amethyst::{
    ecs::{
        storage::DenseVecStorage, Component, Entities, Entity, Read, World, WorldExt, WriteStorage,
    },
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, play::AssetWorld, ItemComponent};
use asset_ui_model::play::AssetSelectionParent;
use derivative::Derivative;
use derive_new::new;
use log::error;
use parent_model::play::ParentEntity;

use crate::play::{MswMain, MswStatus};

/// Tracks the Item IDs to be attached to entities that represent the map selection widget.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct MapSelectionWidget {
    /// Layers of sprite labels to draw for the map selection widget.
    pub layers: Vec<ItemId>,
}

/// `MapSelectionWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSelectionWidgetSystemData<'s> {
    /// `AssetWorld`.
    #[derivative(Debug = "ignore")]
    pub asset_world: Read<'s, AssetWorld>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `MswStatus` components.
    #[derivative(Debug = "ignore")]
    pub msw_statuses: WriteStorage<'s, MswStatus>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
}

impl<'s> ItemComponent<'s> for MapSelectionWidget {
    type SystemData = MapSelectionWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let MapSelectionWidgetSystemData {
            asset_world,
            entities,
            item_ids,
            parent_entities,
            msw_statuses,
            asset_selection_parents,
        } = system_data;

        let msw_main_item = {
            let msw_mains = asset_world.read_storage::<MswMain>();
            self.layers
                .iter()
                .find(|layer_item_id| msw_mains.get(layer_item_id.0).is_some())
                .copied()
        };

        let (msw_main_entity, layer_entities) = self.layers.iter().copied().fold(
            (None, Vec::with_capacity(self.layers.len())),
            |(mut msw_main_entity, mut layer_entities), item_id| {
                let parent_entity = ParentEntity(entity);
                let layer_entity = entities
                    .build_entity()
                    .with(item_id, item_ids)
                    .with(parent_entity, parent_entities)
                    .with(MswStatus::default(), msw_statuses)
                    .build();

                if msw_main_entity.is_none() {
                    if let Some(msw_main_item) = msw_main_item {
                        if item_id == msw_main_item {
                            msw_main_entity = Some(layer_entity);
                        }
                    }
                }
                layer_entities.push(layer_entity);

                (msw_main_entity, layer_entities)
            },
        );

        if let Some(msw_main_entity) = msw_main_entity {
            let asset_selection_parent = AssetSelectionParent::new(msw_main_entity);
            layer_entities
                .iter()
                .filter(|layer_entity| **layer_entity != msw_main_entity)
                .copied()
                .for_each(|layer_entity| {
                    asset_selection_parents
                        .insert(layer_entity, asset_selection_parent)
                        .expect("Failed to insert `AssetSelectionParent` component.");
                })
        } else {
            error!("Expected `MapSelectionWidget` template to have at least one layer.");
        }
    }
}
