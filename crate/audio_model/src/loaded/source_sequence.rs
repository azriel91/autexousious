use sequence_model_derive::frame_component_data;

use crate::loaded::SourceHandleOpt;

/// Sequence for volumes that can be hit.
#[frame_component_data(SourceHandleOpt)]
pub struct SourceSequence;
