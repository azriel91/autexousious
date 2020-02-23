use amethyst::{
    ecs::{storage::DenseVecStorage, Component, Entity, World, WriteStorage},
    shred::{ResourceId, SystemData},
};
use asset_model::ItemComponent;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Width and height of a UI element.
#[derive(Clone, Component, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dimensions {
    /// Width of the element.
    pub w: u32,
    /// Height of the element.
    pub h: u32,
}

/// `DimensionsSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct DimensionsSystemData<'s> {
    /// `Dimensions` components.
    #[derivative(Debug = "ignore")]
    pub dimensionses: WriteStorage<'s, Dimensions>,
}

impl<'s> ItemComponent<'s> for Dimensions {
    type SystemData = DimensionsSystemData<'s>;

    fn augment(&self, system_data: &mut Self::SystemData, entity: Entity) {
        let DimensionsSystemData { dimensionses } = system_data;

        if dimensionses.get(entity).is_none() {
            dimensionses
                .insert(entity, *self)
                .expect("Failed to insert `Dimensions` component.");
        }
    }
}
