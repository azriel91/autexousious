use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{play::AssetSelection, ItemComponent};
use derivative::Derivative;
use derive_new::new;
use ui_model_spi::play::WidgetStatus;

use crate::loaded::{AssetDisplayCell, AssetDisplayCellSystemData};

/// `AssetSelection` cell for a particular asset.
///
/// Essentially an `AssetDisplayCell` with an `AssetSelection` attached.
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub enum AssetSelectionCell {
    /// Cell for an `AssetId` selection.
    Id {
        /// Inner display cell.
        display_cell: AssetDisplayCell,
    },
    /// Cell for `Random` selection.
    Random,
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
    /// `WidgetStatus` components.
    #[derivative(Debug = "ignore")]
    pub widget_statuses: WriteStorage<'s, WidgetStatus>,
}

impl<'s> ItemComponent<'s> for AssetSelectionCell {
    type SystemData = AssetSelectionCellSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectionCellSystemData {
            asset_display_cell_system_data,
            asset_selections,
            widget_statuses,
        } = system_data;

        match self {
            Self::Id { display_cell } => {
                display_cell.augment(asset_display_cell_system_data, entity);
                if !asset_selections.contains(entity) {
                    asset_selections
                        .insert(entity, AssetSelection::Id(display_cell.asset_id))
                        .expect("Failed to insert `AssetSelection` component.");
                }
            }
            Self::Random => {
                if !asset_selections.contains(entity) {
                    asset_selections
                        .insert(entity, AssetSelection::Random)
                        .expect("Failed to insert `AssetSelection` component.");
                }
            }
        }

        // TODO: can this move to `ui_model_spi`?
        if !widget_statuses.contains(entity) {
            widget_statuses
                .insert(entity, WidgetStatus::Idle)
                .expect("Failed to insert `WidgetStatus` component.");
        }
    }
}
