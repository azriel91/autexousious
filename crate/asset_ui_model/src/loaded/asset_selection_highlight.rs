use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use chase_model::play::TargetObject;
use derivative::Derivative;
use derive_new::new;
use game_input::InputControlled;
use parent_model::play::ParentEntity;

use crate::play::{AssetSelectionHighlightMain, AssetSelectionParent, AssetSelectionStatus};

/// Highlights an asset selection.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct AssetSelectionHighlight {
    /// Layers of sprite labels to draw for the character selection widget.
    pub ash_template_item_id: ItemId,
    /// InputControlled to attach to each `AssetSelectionHighlight` sub entity.
    pub input_controlled: InputControlled,
    /// The `AssetSelectionStatus` to begin with.
    ///
    /// For character selection, this would be `Inactive`. For map selection, it would be
    /// `InProgress`.
    pub asset_selection_status: AssetSelectionStatus,
}

/// `AssetSelectionHighlightSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionHighlightSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_highlight_mains: WriteStorage<'s, AssetSelectionHighlightMain>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: WriteStorage<'s, InputControlled>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `AssetSelectionStatus` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_statuses: WriteStorage<'s, AssetSelectionStatus>,
    /// `AssetSelectionParent` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_parents: WriteStorage<'s, AssetSelectionParent>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: WriteStorage<'s, TargetObject>,
}

impl<'s> ItemComponent<'s> for AssetSelectionHighlight {
    type SystemData = AssetSelectionHighlightSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectionHighlightSystemData {
            entities,
            asset_selection_highlight_mains,
            item_ids,
            input_controlleds,
            parent_entities,
            asset_selection_statuses,
            asset_selection_parents,
            target_objects,
        } = system_data;

        if !asset_selection_highlight_mains.contains(entity) {
            asset_selection_highlight_mains
                .insert(entity, AssetSelectionHighlightMain)
                .expect("Failed to insert `AssetSelectionHighlightMain` component.");
        }
        if !input_controlleds.contains(entity) {
            input_controlleds
                .insert(entity, self.input_controlled)
                .expect("Failed to insert `InputControlled` component.");
        }

        let asset_selection_parent = AssetSelectionParent::new(entity);
        let parent_entity = ParentEntity::new(entity);
        let target_object = TargetObject::new(entity);
        entities
            .build_entity()
            .with(self.ash_template_item_id, item_ids)
            .with(self.input_controlled, input_controlleds)
            .with(parent_entity, parent_entities)
            .with(self.asset_selection_status, asset_selection_statuses)
            .with(asset_selection_parent, asset_selection_parents)
            .with(target_object, target_objects)
            .build();
    }
}
