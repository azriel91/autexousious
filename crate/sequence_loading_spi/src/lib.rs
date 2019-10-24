#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes sequence configuration into the loaded model.

pub use crate::{
    frame_component_data_loader::FrameComponentDataLoader,
    sequence_component_data_loader::SequenceComponentDataLoader,
};

mod frame_component_data_loader;
mod sequence_component_data_loader;
