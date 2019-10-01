use amethyst::ecs::{storage::DenseVecStorage, Component};
use sequence_model_derive::sequence_component_data;

use crate::loaded::SequenceEndTransition;

/// Sequence transition upon sequence end.
#[sequence_component_data(SequenceEndTransition)]
#[derive(Component)]
pub struct SequenceEndTransitions;
