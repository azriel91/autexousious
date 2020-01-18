use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::{play::AssetSelection, ItemComponent};
use derivative::Derivative;

/// Marks an entity as the main character selection widget entity.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct CswMain;

/// `CswMainSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CswMainSystemData<'s> {
    /// `CswMain` components.
    #[derivative(Debug = "ignore")]
    pub csw_mains: WriteStorage<'s, CswMain>,
    /// `AssetSelection` components.
    #[derivative(Debug = "ignore")]
    pub asset_selections: WriteStorage<'s, AssetSelection>,
}

impl<'s> ItemComponent<'s> for CswMain {
    type SystemData = CswMainSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let CswMainSystemData {
            csw_mains,
            asset_selections,
        } = system_data;

        if csw_mains.get(entity).is_none() {
            csw_mains
                .insert(entity, CswMain)
                .expect("Failed to insert `CswMain` component.");
        }
        if asset_selections.get(entity).is_none() {
            asset_selections
                .insert(entity, AssetSelection::Random)
                .expect("Failed to insert `AssetSelection` component.");
        }
    }
}
