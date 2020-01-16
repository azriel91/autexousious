use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Marks an entity as the main `AssetSelectionHighlightMain` entity.
#[derive(Clone, Component, Copy, Debug, Default)]
#[storage(NullStorage)]
pub struct AssetSelectionHighlightMain;

/// `AssetSelectionHighlightMainSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionHighlightMainSystemData<'s> {
    /// `AssetSelectionHighlightMain` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_highlight_mains: WriteStorage<'s, AssetSelectionHighlightMain>,
}

impl<'s> ItemComponent<'s> for AssetSelectionHighlightMain {
    type SystemData = AssetSelectionHighlightMainSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectionHighlightMainSystemData {
            asset_selection_highlight_mains,
        } = system_data;

        if asset_selection_highlight_mains.get(entity).is_none() {
            asset_selection_highlight_mains
                .insert(entity, *self)
                .expect("Failed to insert `AssetSelectionHighlightMain` component.");
        }
    }
}
