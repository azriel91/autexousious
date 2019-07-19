use sequence_model::config::SequenceId;
use sequence_model_derive::sequence_component_data;

use crate::loaded::BodySequenceHandle;

/// Map of `BodySequenceHandle`s, keyed by Sequence ID.
#[sequence_component_data(SeqId, BodySequenceHandle)]
pub struct BodySequenceHandles<SeqId>
where
    SeqId: SequenceId;
