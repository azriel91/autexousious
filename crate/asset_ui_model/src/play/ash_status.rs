use amethyst::{
    ecs::{storage::VecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Asset selection status of the `AssetSelectionHighlightMain` entity.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum AshStatus {
    /// Widget is inactive.
    #[derivative(Default)]
    Inactive,
    /// Asset is being selected.
    AssetSelect,
    /// Selection has been confirmed.
    Ready,
}

/// `AshStatusSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AshStatusSystemData<'s> {
    /// `AshStatus` components.
    #[derivative(Debug = "ignore")]
    pub ash_statuses: WriteStorage<'s, AshStatus>,
}

impl<'s> ItemComponent<'s> for AshStatus {
    type SystemData = AshStatusSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let AshStatusSystemData { ash_statuses } = system_data;

        if ash_statuses.get(entity).is_none() {
            ash_statuses
                .insert(entity, *self)
                .expect("Failed to insert `AshStatus` component.");
        }
    }
}
