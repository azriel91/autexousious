use crate::{component_sequence, config::Wait};

/// Sequence of `Wait` values.
#[component_sequence(Wait, copy)]
pub struct WaitSequence;
