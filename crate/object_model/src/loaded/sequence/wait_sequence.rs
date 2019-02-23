use object_model_spi::loaded::ComponentFrames;

use crate::config::object::Wait;

/// Sequence of `Wait` values.
pub type WaitSequence = ComponentFrames<Wait>;
