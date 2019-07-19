use sequence_model::config::SequenceId;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SpawnsSequenceHandle;

/// Map of `SpawnsSequenceHandle`s, keyed by Sequence ID.
#[sequence_component_data(SeqId, SpawnsSequenceHandle)]
pub struct SpawnsSequenceHandles<SeqId>
where
    SeqId: SequenceId;
