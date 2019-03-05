use sequence_model_spi::loaded::ComponentFrames;

use crate::config::Wait;

/// Sequence of `Wait` values.
pub type WaitSequence = ComponentFrames<Wait>;
