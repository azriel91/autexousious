use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Selection status of the asset selection widget.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum AssetSelectionStatus {
    /// Asset selection is deactivated.
    ///
    /// Useful for character selection when the player has not joined.
    #[derivative(Default)]
    Inactive,
    /// Asset is being selected.
    InProgress,
    /// Selection has been confirmed.
    Ready,
}

/// `AssetSelectionStatusSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetSelectionStatusSystemData<'s> {
    /// `AssetSelectionStatus` components.
    #[derivative(Debug = "ignore")]
    pub asset_selection_statuses: WriteStorage<'s, AssetSelectionStatus>,
}

impl<'s> ItemComponent<'s> for AssetSelectionStatus {
    type SystemData = AssetSelectionStatusSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AssetSelectionStatusSystemData {
            asset_selection_statuses,
        } = system_data;

        if asset_selection_statuses.get(entity).is_none() {
            asset_selection_statuses
                .insert(entity, *self)
                .expect("Failed to insert `AssetSelectionStatus` component.");
        }
    }
}
