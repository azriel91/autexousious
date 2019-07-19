use sequence_model_derive::sequence_component_data;

use crate::{config::SequenceId, loaded::WaitSequenceHandle};

/// Map of `WaitSequenceHandle`s, keyed by Sequence ID.
#[sequence_component_data(SeqId, WaitSequenceHandle)]
pub struct WaitSequenceHandles<SeqId>
where
    SeqId: SequenceId;
