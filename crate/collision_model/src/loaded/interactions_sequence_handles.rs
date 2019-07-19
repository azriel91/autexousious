use sequence_model::config::SequenceId;
use sequence_model_derive::sequence_component_data;

use crate::loaded::InteractionsSequenceHandle;

/// Map of `InteractionsSequenceHandle`s, keyed by Sequence ID.
#[sequence_component_data(SeqId, InteractionsSequenceHandle)]
pub struct InteractionsSequenceHandles<SeqId>
where
    SeqId: SequenceId;
