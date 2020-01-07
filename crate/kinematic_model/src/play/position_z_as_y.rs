use amethyst::{
    ecs::{storage::NullStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;

/// Indicates the Z position should be rendered as part of the Y transform.
#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct PositionZAsY;

/// `PositionZAsYSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct PositionZAsYSystemData<'s> {
    /// `PositionZAsY` components.
    #[derivative(Debug = "ignore")]
    pub position_z_as_ys: WriteStorage<'s, PositionZAsY>,
}

impl<'s> ItemComponent<'s> for PositionZAsY {
    type SystemData = PositionZAsYSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let PositionZAsYSystemData { position_z_as_ys } = system_data;

        position_z_as_ys
            .insert(entity, PositionZAsY)
            .expect("Failed to insert `PositionZAsY` component.");
    }
}
