use sequence_model_derive::sequence_component_data;

use crate::config::{SequenceEndTransition, SequenceId};

/// Sequence of sequence transitions upon sequence end.
#[sequence_component_data(SeqId, SequenceEndTransition<SeqId>)]
pub struct SequenceEndTransitions<SeqId>
where
    SeqId: SequenceId;
