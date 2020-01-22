use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{play::AssetSelection, ItemComponent};
use derivative::Derivative;

/// Marks an entity as the main asset preview widget entity.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct ApwMain;

/// `ApwMainSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ApwMainSystemData<'s> {
    /// `ApwMain` components.
    #[derivative(Debug = "ignore")]
    pub apw_mains: WriteStorage<'s, ApwMain>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: WriteStorage<'s, AssetSelection>,
}

impl<'s> ItemComponent<'s> for ApwMain {
    type SystemData = ApwMainSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let ApwMainSystemData {
            apw_mains,
            asset_selections,
        } = system_data;

        if apw_mains.get(entity).is_none() {
            apw_mains
                .insert(entity, ApwMain)
                .expect("Failed to insert `ApwMain` component.");
        }
        if asset_selections.get(entity).is_none() {
            asset_selections
                .insert(entity, AssetSelection::Random)
                .expect("Failed to insert `AssetSelection` component.");
        }
    }
}
