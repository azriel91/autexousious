use sequence_loading_spi::FrameComponentDataLoader;
use sequence_model::{config::Wait, loaded::WaitSequence};

/// Loads `WaitSequence`s from `Sequence` types whose `Frame`s contain a `Wait` value.
#[derive(Debug)]
pub struct WaitSequenceLoader;

impl FrameComponentDataLoader for WaitSequenceLoader {
    type Component = Wait;
    type ComponentData = WaitSequence;
}
