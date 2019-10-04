#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic to update sequences.

pub use crate::system::{
    FrameComponentUpdateSystem, FrameComponentUpdateSystemData, SequenceComponentUpdateSystem,
    SequenceComponentUpdateSystemData, SequenceEndTransitionSystem,
    SequenceEndTransitionSystemData, SequenceStatusUpdateSystem, SequenceStatusUpdateSystemData,
    SequenceUpdateSystem, SequenceUpdateSystemData,
};

mod system;
