use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::WaitSequenceHandle;

/// Sequence of `WaitSequenceHandle`s.
#[sequence_component_data(WaitSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct WaitSequenceHandles;
