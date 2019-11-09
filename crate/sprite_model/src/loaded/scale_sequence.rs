use sequence_model::frame_component_data;

use crate::config::Scale;

/// Sequence of `Scale` values.
#[frame_component_data(Scale, copy)]
pub struct ScaleSequence;
