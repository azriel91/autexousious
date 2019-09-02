#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides systems that update the map during game play.

pub use crate::system::{
    KeepWithinMapBoundsSystem, MapEnterExitDetectionSystem, MapOutOfBoundsClockAugmentSystem,
    MapOutOfBoundsDeletionSystem,
};

mod system;
