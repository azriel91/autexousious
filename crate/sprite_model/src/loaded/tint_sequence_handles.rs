use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::TintSequenceHandle;

/// Sequence of `TintSequenceHandle`s.
#[sequence_component_data(TintSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct TintSequenceHandles;
