use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{play::AssetSelection, ItemComponent};
use derivative::Derivative;

/// Marks an entity as the main map selection widget entity.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct MswMain;

/// `MswMainSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MswMainSystemData<'s> {
    /// `MswMain` components.
    #[derivative(Debug = "ignore")]
    pub msw_mains: WriteStorage<'s, MswMain>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: WriteStorage<'s, AssetSelection>,
}

impl<'s> ItemComponent<'s> for MswMain {
    type SystemData = MswMainSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let MswMainSystemData {
            msw_mains,
            asset_selections,
        } = system_data;

        if msw_mains.get(entity).is_none() {
            msw_mains
                .insert(entity, MswMain)
                .expect("Failed to insert `MswMain` component.");
        }
        if asset_selections.get(entity).is_none() {
            asset_selections
                .insert(entity, AssetSelection::Random)
                .expect("Failed to insert `AssetSelection` component.");
        }
    }
}
