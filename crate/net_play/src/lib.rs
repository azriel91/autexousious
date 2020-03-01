#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for the network.

pub use crate::system::{NetListenerSystem, NetListenerSystemDesc};

mod system;
