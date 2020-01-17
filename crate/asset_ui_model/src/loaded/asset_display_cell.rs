use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{loaded::AssetId, ItemComponent};
use derivative::Derivative;
use derive_new::new;

/// Display cell for a particular asset.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetDisplayCell {
    /// ID of the asset to display.
    pub asset_id: AssetId,
}

/// `AssetDisplayCellSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDisplayCellSystemData<'s> {
    /// `AssetDisplayCell` components.
    #[derivative(Debug = "ignore")]
    pub asset_display_cells: WriteStorage<'s, AssetDisplayCell>,
}

impl<'s> ItemComponent<'s> for AssetDisplayCell {
    type SystemData = AssetDisplayCellSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetDisplayCellSystemData {
            asset_display_cells,
        } = system_data;

        if !asset_display_cells.contains(entity) {
            asset_display_cells
                .insert(entity, *self)
                .expect("Failed to insert `AssetDisplayCell` component.");
        }
    }
}
