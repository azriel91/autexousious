use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SourceSequenceHandle;

/// Sequence of `SourceSequenceHandle`s.
#[sequence_component_data(SourceSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct SourceSequenceHandles;
