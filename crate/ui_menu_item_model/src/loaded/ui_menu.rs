use std::fmt::Debug;

use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use parent_model::play::ParentEntity;
use ui_model_spi::play::SiblingsBoundaryAction;
use ui_model_spi_play::{UiRectifySystemData, UiWidgetRectifier};

/// Displays menu items.
#[derive(Clone, Component, Debug, Default, PartialEq, new)]
pub struct UiMenu {
    /// `ItemId`s of the `UiMenuItems` to instantiate.
    ///
    /// Note: These must be sorted in the order of the items top to bottom.
    pub ui_menu_item_item_ids: Vec<ItemId>,
    /// `ItemId`s of the sprite items to instantiate.
    ///
    /// Note: These must be sorted in the order of the items top to bottom.
    pub sprite_item_ids: Vec<ItemId>,
}

/// `UiMenuSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiMenuSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `UiRectifySystemData` components.
    #[derivative(Debug = "ignore")]
    pub ui_rectify_system_data: UiRectifySystemData<'s>,
}

impl UiMenu {
    fn augment_siblings(
        &self,
        ui_rectify_system_data: &mut UiRectifySystemData,
        ui_menu_item_entities: &[Entity],
    ) {
        UiWidgetRectifier::rectify(
            ui_rectify_system_data,
            SiblingsBoundaryAction::CycleNext,
            ui_menu_item_entities,
        )
    }
}

impl<'s> ItemComponent<'s> for UiMenu {
    type SystemData = UiMenuSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiMenuSystemData {
            entities,
            item_ids,
            parent_entities,
            ui_rectify_system_data,
        } = system_data;

        let parent_entity = ParentEntity::new(entity);

        let ui_menu_item_entities = self
            .ui_menu_item_item_ids
            .iter()
            .copied()
            .map(|ui_menu_item_item_id| {
                entities
                    .build_entity()
                    .with(ui_menu_item_item_id, item_ids)
                    .with(parent_entity, parent_entities)
                    .build()
            })
            .collect::<Vec<Entity>>();

        // Add `Siblings` component for each `UiMenuItem` for navigation.
        self.augment_siblings(ui_rectify_system_data, &ui_menu_item_entities);

        self.sprite_item_ids
            .iter()
            .copied()
            .for_each(|sprite_item_item_id| {
                entities
                    .build_entity()
                    .with(sprite_item_item_id, item_ids)
                    .with(parent_entity, parent_entities)
                    .build();
            });
    }
}
