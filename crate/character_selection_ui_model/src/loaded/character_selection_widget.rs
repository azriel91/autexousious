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
use game_input::InputControlled;
use log::error;
use parent_model::play::ParentEntity;

use crate::play::{CswMain, CswStatus};

/// Tracks the Item IDs to be attached to entities that represent the character selection widget.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct CharacterSelectionWidget {
    /// Layers of sprite labels to draw for the character selection widget.
    pub layers: Vec<ItemId>,
    /// InputControlled to attach to each layer entity.
    pub input_controlled: InputControlled,
}

/// `CharacterSelectionWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionWidgetSystemData<'s> {
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
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `CswStatus` components.
    #[derivative(Debug = "ignore")]
    pub csw_statuses: WriteStorage<'s, CswStatus>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
}

impl<'s> ItemComponent<'s> for CharacterSelectionWidget {
    type SystemData = CharacterSelectionWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let CharacterSelectionWidgetSystemData {
            asset_world,
            entities,
            item_ids,
            input_controlleds,
            parent_entities,
            csw_statuses,
            asset_selection_parents,
        } = system_data;

        let csw_main_item = {
            let csw_mains = asset_world.read_storage::<CswMain>();
            self.layers
                .iter()
                .find(|layer_item_id| csw_mains.get(layer_item_id.0).is_some())
                .copied()
        };

        let (csw_main_entity, layer_entities) = self.layers.iter().copied().fold(
            (None, Vec::with_capacity(self.layers.len())),
            |(mut csw_main_entity, mut layer_entities), item_id| {
                let parent_entity = ParentEntity(entity);
                let layer_entity = entities
                    .build_entity()
                    .with(item_id, item_ids)
                    .with(self.input_controlled, input_controlleds)
                    .with(parent_entity, parent_entities)
                    .with(CswStatus::default(), csw_statuses)
                    .build();

                if csw_main_entity.is_none() {
                    if let Some(csw_main_item) = csw_main_item {
                        if item_id == csw_main_item {
                            csw_main_entity = Some(layer_entity);
                        }
                    }
                }
                layer_entities.push(layer_entity);

                (csw_main_entity, layer_entities)
            },
        );

        if let Some(csw_main_entity) = csw_main_entity {
            let asset_selection_parent = AssetSelectionParent::new(csw_main_entity);
            layer_entities
                .iter()
                .filter(|layer_entity| **layer_entity != csw_main_entity)
                .copied()
                .for_each(|layer_entity| {
                    asset_selection_parents
                        .insert(layer_entity, asset_selection_parent)
                        .expect("Failed to insert `AssetSelectionParent` component.");
                })
        } else {
            error!("Expected `CharacterSelectionWidget` template to have at least one layer.");
        }
    }
}
