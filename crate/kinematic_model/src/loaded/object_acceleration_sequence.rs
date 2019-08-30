use sequence_model_derive::frame_component_data;

use crate::config::ObjectAcceleration;

/// Sequence of `ObjectAcceleration` values.
#[frame_component_data(ObjectAcceleration)]
pub struct ObjectAccelerationSequence;
