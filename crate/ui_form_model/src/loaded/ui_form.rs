use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
    ui::Selectable,
};
use asset_model::{loaded::ItemId, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use parent_model::play::ParentEntity;

/// Displays a form that takes in input.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct UiForm {
    /// `ItemId`s of the `UiLabel`s, `UiSpriteLabel`s and `UiTextInput` for each form item.
    pub ui_form_item_item_ids: Vec<(ItemId, ItemId, ItemId)>,
}

/// `UiFormSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiFormSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `Selectable<()>` components.
    ///
    /// `Note:` The `UiBundle` defaults the selectable group `G` type parameter to `()`.
    #[derivative(Debug = "ignore")]
    pub selectables: WriteStorage<'s, Selectable<()>>,
}

impl<'s> ItemComponent<'s> for UiForm {
    type SystemData = UiFormSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let UiFormSystemData {
            entities,
            item_ids,
            parent_entities,
            selectables,
        } = system_data;

        let parent_entity = ParentEntity::new(entity);

        self.ui_form_item_item_ids
            .iter()
            .copied()
            .enumerate()
            .for_each(
                |(index, (label_item_id, sprite_item_id, input_field_item_id))| {
                    entities
                        .build_entity()
                        .with(label_item_id, item_ids)
                        .with(parent_entity, parent_entities)
                        .build();

                    entities
                        .build_entity()
                        .with(sprite_item_id, item_ids)
                        .with(parent_entity, parent_entities)
                        .build();

                    let selectable = Selectable::new(index as u32);
                    entities
                        .build_entity()
                        .with(input_field_item_id, item_ids)
                        .with(parent_entity, parent_entities)
                        .with(selectable, selectables)
                        .build();
                },
            );
    }
}
