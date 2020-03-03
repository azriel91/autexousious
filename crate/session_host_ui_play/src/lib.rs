#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for session host UI at runtime.

pub use crate::system::{SessionStatusHostUiSystem, SessionStatusHostUiSystemDesc};

mod system;
