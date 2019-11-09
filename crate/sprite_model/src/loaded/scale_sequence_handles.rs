use sequence_model_derive::sequence_component_data;

use crate::loaded::ScaleSequenceHandle;

/// Sequence of `ScaleSequenceHandle`s.
#[sequence_component_data(ScaleSequenceHandle)]
pub struct ScaleSequenceHandles;
