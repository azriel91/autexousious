#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic to update sequences.

pub use crate::system::{
    FrameComponentUpdateSystem, SequenceEndTransitionSystem, SequenceStatusUpdateSystem,
    SequenceUpdateSystem,
};

mod system;
