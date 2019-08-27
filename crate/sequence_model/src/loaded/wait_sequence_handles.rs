use sequence_model_derive::sequence_component_data;

use crate::loaded::WaitSequenceHandle;

/// Map of `WaitSequenceHandle`s.
#[sequence_component_data(WaitSequenceHandle)]
pub struct WaitSequenceHandles;
