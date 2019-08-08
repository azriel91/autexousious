#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for game objects used during game play.

pub use crate::system::{ChargeInitializeDelaySystem, ChargeInitializeDetectionSystem};

mod system;
