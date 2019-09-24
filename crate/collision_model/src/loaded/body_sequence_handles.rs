use sequence_model_derive::sequence_component_data;

use crate::loaded::BodySequenceHandle;

/// Sequence of `BodySequenceHandle`s.
#[sequence_component_data(BodySequenceHandle)]
pub struct BodySequenceHandles;
