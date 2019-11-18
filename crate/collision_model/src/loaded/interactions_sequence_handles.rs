use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::InteractionsSequenceHandle;

/// Sequence of `InteractionsSequenceHandle`s.
#[sequence_component_data(InteractionsSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct InteractionsSequenceHandles;
