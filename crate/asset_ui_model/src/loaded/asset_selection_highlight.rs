use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use chase_model::play::TargetObject;
use derivative::Derivative;
use derive_new::new;
use parent_model::play::ParentEntity;

use crate::play::{AssetSelectionParent, AssetSelectionStatus};

/// Highlights an asset selection.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct AssetSelectionHighlight {
    /// `ItemId` of sprite to draw for the character selection widget.
    pub ash_sprite_item_id: ItemId,
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
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
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
            item_ids,
            parent_entities,
            asset_selection_statuses,
            asset_selection_parents,
            target_objects,
        } = system_data;

        let asset_selection_parent = AssetSelectionParent::new(entity);
        let parent_entity = ParentEntity::new(entity);
        let target_object = TargetObject::new(entity);
        // `ChaseModeStick` should be inserted by the `AssetSelectionHighlight` sprite `ItemId`.
        entities
            .build_entity()
            .with(self.ash_sprite_item_id, item_ids)
            .with(parent_entity, parent_entities)
            .with(self.asset_selection_status, asset_selection_statuses)
            .with(asset_selection_parent, asset_selection_parents)
            .with(target_object, target_objects)
            .build();
    }
}
