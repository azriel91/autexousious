#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic to update sequences.

pub use crate::system::{
    FrameComponentUpdateSystem, SequenceComponentUpdateSystem, SequenceEndTransitionSystem,
    SequenceStatusUpdateSystem, SequenceUpdateSystem,
};

mod system;
