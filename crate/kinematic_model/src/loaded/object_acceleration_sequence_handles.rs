use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::ObjectAccelerationSequenceHandle;

/// Sequence of `ObjectAccelerationSequenceHandle`s.
#[sequence_component_data(ObjectAccelerationSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct ObjectAccelerationSequenceHandles;

impl<'s> ItemComponent<'s> for ObjectAccelerationSequenceHandles {
    type SystemData = ();
}
