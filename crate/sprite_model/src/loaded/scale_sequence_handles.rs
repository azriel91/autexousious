use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::ScaleSequenceHandle;

/// Sequence of `ScaleSequenceHandle`s.
#[sequence_component_data(ScaleSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct ScaleSequenceHandles;
