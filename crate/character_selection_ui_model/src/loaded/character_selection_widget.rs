use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use log::error;

use crate::play::CharacterSelectionParent;

/// Tracks the Item IDs to be attached to entities that represent the character selection widget.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct CharacterSelectionWidget {
    /// Layers of sprite labels to draw for the character selection widget.
    pub layers: Vec<ItemId>,
}

/// `CharacterSelectionWidgetSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterSelectionWidgetSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `CharacterSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub character_selection_parents: WriteStorage<'s, CharacterSelectionParent>,
}

impl<'s> ItemComponent<'s> for CharacterSelectionWidget {
    type SystemData = CharacterSelectionWidgetSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, _entity: Entity) {
        let CharacterSelectionWidgetSystemData {
            entities,
            item_ids,
            character_selection_parents,
        } = system_data;

        let layer_entities = self
            .layers
            .iter()
            .copied()
            .map(|item_id| {
                // TODO: let parent_entity = ParentEntity(self);
                entities
                    .build_entity()
                    // TODO: .with(parent_entity, parent_entities)
                    .with(item_id, item_ids)
                    .build()
            })
            .collect::<Vec<Entity>>();

        let first_layer_entity = layer_entities.first().copied();

        if let Some(first_layer_entity) = first_layer_entity {
            let character_selection_parent = CharacterSelectionParent::new(first_layer_entity);
            layer_entities
                .iter()
                .skip(1)
                .copied()
                .for_each(|layer_entity| {
                    character_selection_parents
                        .insert(layer_entity, character_selection_parent)
                        .expect("Failed to insert `CharacterSelectionParent` component.");
                })
        } else {
            error!("Expected `CharacterSelectionWidget` template to have at least one layer.");
        }
    }
}
