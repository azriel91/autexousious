use amethyst::ecs::{storage::DenseVecStorage, Component};
use sequence_model_derive::sequence_component_data;
use specs_derive::Component;

use crate::config::{SequenceEndTransition, SequenceId};

/// Sequence of sequence transitions upon sequence end.
#[sequence_component_data(SeqId, SequenceEndTransition<SeqId>)]
#[derive(Component)]
pub struct SequenceEndTransitions<SeqId>
where
    SeqId: SequenceId;
