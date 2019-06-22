#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides spawn logic during game play.

pub use crate::system::{SpawnGameObjectRectifySystem, SpawnGameObjectSystem};

mod system;
