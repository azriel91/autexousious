use crate::{config::Wait, frame_component_data};

/// Sequence of `Wait` values.
#[frame_component_data(Wait, copy)]
pub struct WaitSequence;
