use sequence_model_derive::sequence_component_data;

use crate::loaded::TintSequenceHandle;

/// Sequence of `TintSequenceHandle`s.
#[sequence_component_data(TintSequenceHandle)]
pub struct TintSequenceHandles;
