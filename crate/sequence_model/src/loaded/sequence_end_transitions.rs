use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SequenceEndTransition;

/// Sequence transition upon sequence end.
#[sequence_component_data(SequenceEndTransition)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct SequenceEndTransitions;
