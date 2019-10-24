use sequence_loading_spi::SequenceComponentDataLoader;
use sequence_model::loaded::{WaitSequenceHandle, WaitSequenceHandles};

/// Loads `WaitSequenceHandle`s from collections of sequences that contain `Wait` values.
#[derive(Debug)]
pub struct WaitSequenceHandlesLoader;

impl SequenceComponentDataLoader for WaitSequenceHandlesLoader {
    type Component = WaitSequenceHandle;
    type ComponentData = WaitSequenceHandles;
}
