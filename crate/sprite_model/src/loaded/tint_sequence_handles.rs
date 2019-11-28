use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::TintSequenceHandle;

/// Sequence of `TintSequenceHandle`s.
#[sequence_component_data(TintSequenceHandle)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct TintSequenceHandles;

impl<'s> ItemComponent<'s> for TintSequenceHandles {
    type SystemData = ();
}
