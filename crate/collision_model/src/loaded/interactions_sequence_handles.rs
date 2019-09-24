use sequence_model_derive::sequence_component_data;

use crate::loaded::InteractionsSequenceHandle;

/// Sequence of `InteractionsSequenceHandle`s.
#[sequence_component_data(InteractionsSequenceHandle)]
pub struct InteractionsSequenceHandles;
