use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::BodySequenceHandle;

/// Sequence of `BodySequenceHandle`s.
#[sequence_component_data(BodySequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct BodySequenceHandles;

impl<'s> ItemComponent<'s> for BodySequenceHandles {
    type SystemData = ();
}
