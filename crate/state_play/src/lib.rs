#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides behaviour logic for states.

pub use crate::system::{
    StateBackgroundSpawnSystem, StateBackgroundSpawnSystemData, StateCameraResetSystem,
    StateCameraResetSystemData, StateIdEventSystem, StateIdEventSystemData,
};

mod system;
