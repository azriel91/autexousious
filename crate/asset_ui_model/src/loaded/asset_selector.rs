use std::marker::PhantomData;

use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entities, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::ItemId, ItemComponent};
use chase_model::play::TargetObject;
use derivative::Derivative;
use derive_new::new;
use log::warn;
use parent_model::play::ParentEntity;
use ui_model_spi::play::SiblingsVertical;
use ui_model_spi_play::{UiRectifySystemData, UiWidgetRectifier};

use crate::config::{AssetDisplayGrid, AssetDisplayLayout};

/// Displays available assets and highlights selected asset.
///
/// # Type Parameters
///
/// * `T`: Type to indicate the assets to display.
#[derive(Clone, Component, Debug, PartialEq, new)]
pub struct AssetSelector<T>
where
    T: Send + Sync + 'static,
{
    /// `ItemId`s of the `AssetDisplayCell`s to instantiate.
    ///
    /// Note: These must be sorted in the order of the cells left to right, top to bottom.
    pub asset_display_cell_item_ids: Vec<ItemId>,
    /// `ItemId`s of the `AssetSelectorHighlight`s to instantiate.
    pub asset_selection_highlight_item_ids: Vec<ItemId>,
    /// Layout of the asset display cells.
    ///
    /// This is used to determine the siblings of the `AssetDisplayCell` entities.
    pub layout: AssetDisplayLayout,
    /// Marker.
    pub marker: PhantomData<T>,
}

/// `AssetSelectorSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectorSystemData<'s> {
    /// `Entities`.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ItemId` components.
    #[derivative(Debug = "ignore")]
    pub item_ids: WriteStorage<'s, ItemId>,
    /// `ParentEntity` components.
    #[derivative(Debug = "ignore")]
    pub parent_entities: WriteStorage<'s, ParentEntity>,
    /// `TargetObject` components.
    #[derivative(Debug = "ignore")]
    pub target_objects: WriteStorage<'s, TargetObject>,
    /// `UiRectifySystemData` components.
    #[derivative(Debug = "ignore")]
    pub ui_rectify_system_data: UiRectifySystemData<'s>,
    /// `SiblingsVertical` components.
    #[derivative(Debug = "ignore")]
    pub siblings_verticals: WriteStorage<'s, SiblingsVertical>,
}

impl<T> AssetSelector<T>
where
    T: Send + Sync + 'static,
{
    fn augment_siblings(
        &self,
        ui_rectify_system_data: &mut UiRectifySystemData,
        _siblings_verticals: &mut WriteStorage<'_, SiblingsVertical>,
        asset_display_cell_entities: &[Entity],
    ) {
        match self.layout {
            AssetDisplayLayout::Grid(AssetDisplayGrid {
                column_count: _, ..
            }) => UiWidgetRectifier::rectify(ui_rectify_system_data, asset_display_cell_entities),
        }
    }
}

impl<'s, T> ItemComponent<'s> for AssetSelector<T>
where
    T: Send + Sync + 'static,
{
    type SystemData = AssetSelectorSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectorSystemData {
            entities,
            item_ids,
            parent_entities,
            target_objects,
            ui_rectify_system_data,
            siblings_verticals,
        } = system_data;

        let parent_entity = ParentEntity::new(entity);

        let asset_display_cell_entities = self
            .asset_display_cell_item_ids
            .iter()
            .copied()
            .map(|asset_display_cell_item_id| {
                entities
                    .build_entity()
                    .with(asset_display_cell_item_id, item_ids)
                    .with(parent_entity, parent_entities)
                    .build()
            })
            .collect::<Vec<Entity>>();

        // Add `Siblings` and `SiblingsVertical` components for each `AssetDisplayCell` for
        // navigation.
        self.augment_siblings(
            ui_rectify_system_data,
            siblings_verticals,
            &asset_display_cell_entities,
        );

        if let Some(first_asset_display_cell) = asset_display_cell_entities.first().copied() {
            // Set `AssetSelectionHighlight` target to first `AssetSelectorHighlight` entity.
            let target_object = TargetObject::new(first_asset_display_cell);
            self.asset_selection_highlight_item_ids
                .iter()
                .copied()
                .for_each(|asset_selection_highlight_item_id| {
                    entities
                        .build_entity()
                        .with(asset_selection_highlight_item_id, item_ids)
                        .with(parent_entity, parent_entities)
                        .with(target_object, target_objects)
                        .build();
                });
        } else {
            warn!(
                "No `AssetDisplayCell`s present. Not setting `AssetSelectionHighlight` positions."
            );
        }
    }
}
