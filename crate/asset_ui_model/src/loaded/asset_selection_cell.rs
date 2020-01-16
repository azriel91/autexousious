use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{play::AssetSelection, ItemComponent};
use derivative::Derivative;
use derive_new::new;

use crate::loaded::{AssetDisplayCell, AssetDisplayCellSystemData};

/// `AssetSelection` cell for a particular asset.
///
/// Essentially an `AssetDisplayCell` with an `AssetSelection` attached.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct AssetSelectionCell {
    /// Inner display cell.
    pub display_cell: AssetDisplayCell,
}

/// `AssetSelectionCellSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionCellSystemData<'s> {
    /// `AssetDisplayCellSystemData`.
    pub asset_display_cell_system_data: AssetDisplayCellSystemData<'s>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: WriteStorage<'s, AssetSelection>,
}

impl<'s> ItemComponent<'s> for AssetSelectionCell {
    type SystemData = AssetSelectionCellSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectionCellSystemData {
            asset_display_cell_system_data,
            asset_selections,
        } = system_data;

        self.display_cell
            .augment(asset_display_cell_system_data, entity);

        if !asset_selections.contains(entity) {
            asset_selections
                .insert(entity, AssetSelection::Random)
                .expect("Failed to insert `AssetSelection` component.");
        }
    }
}
