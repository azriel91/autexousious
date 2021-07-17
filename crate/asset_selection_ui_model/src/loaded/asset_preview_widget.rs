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
use game_input_model::play::{InputControlled, SharedInputControlled};
use log::error;
use parent_model::play::ParentEntity;

use crate::play::ApwMain;

/// Tracks the Item IDs to be attached to entities that represent the asset
/// preview widget.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct AssetPreviewWidget {
    /// Layers of sprite labels to draw for the asset preview widget.
    pub layers: Vec<ItemId>,
}

/// `AssetPreviewWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetPreviewWidgetSystemData<'s> {
    /// `AssetWorld`.
    #[derivative(Debug = "ignore")]
    pub asset_world: Read<'s, AssetWorld>,
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: WriteStorage<'s, SharedInputControlled>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
}

impl<'s> ItemComponent<'s> for AssetPreviewWidget {
    type SystemData = AssetPreviewWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetPreviewWidgetSystemData {
            asset_world,
            entities,
            item_ids,
            input_controlleds,
            shared_input_controlleds,
            parent_entities,
            asset_selection_parents,
        } = system_data;

        let apw_main_item = {
            let apw_mains = asset_world.read_storage::<ApwMain>();
            self.layers
                .iter()
                .find(|layer_item_id| apw_mains.get(layer_item_id.0).is_some())
                .copied()
        };

        let input_controlled = input_controlleds.get(entity).copied();
        let shared_input_controlled = shared_input_controlleds.get(entity).is_some();

        let (apw_main_entity, layer_entities) = self.layers.iter().copied().fold(
            (None, Vec::with_capacity(self.layers.len())),
            |(mut apw_main_entity, mut layer_entities), item_id| {
                let parent_entity = ParentEntity(entity);
                let mut layer_entity_builder = entities
                    .build_entity()
                    .with(item_id, item_ids)
                    .with(parent_entity, parent_entities);

                if let Some(input_controlled) = input_controlled {
                    layer_entity_builder =
                        layer_entity_builder.with(input_controlled, input_controlleds);
                }
                if shared_input_controlled {
                    layer_entity_builder =
                        layer_entity_builder.with(SharedInputControlled, shared_input_controlleds);
                }

                let layer_entity = layer_entity_builder.build();

                if apw_main_entity.is_none() {
                    if let Some(apw_main_item) = apw_main_item {
                        if item_id == apw_main_item {
                            apw_main_entity = Some(layer_entity);
                        }
                    }
                }
                layer_entities.push(layer_entity);

                (apw_main_entity, layer_entities)
            },
        );

        if let Some(apw_main_entity) = apw_main_entity {
            let asset_selection_parent = AssetSelectionParent::new(apw_main_entity);
            layer_entities
                .iter()
                .filter(|layer_entity| **layer_entity != apw_main_entity)
                .copied()
                .for_each(|layer_entity| {
                    asset_selection_parents
                        .insert(layer_entity, asset_selection_parent)
                        .expect("Failed to insert `AssetSelectionParent` component.");
                })
        } else {
            error!("Expected `AssetPreviewWidget` template to have at least one layer.");
        }
    }
}
