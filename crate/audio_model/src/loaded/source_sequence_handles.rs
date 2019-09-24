use sequence_model_derive::sequence_component_data;

use crate::loaded::SourceSequenceHandle;

/// Sequence of `SourceSequenceHandle`s.
#[sequence_component_data(SourceSequenceHandle)]
pub struct SourceSequenceHandles;
