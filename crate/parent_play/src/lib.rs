#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for entity parenting used at runtime.

pub use crate::system::ChildEntityDeleteSystem;

mod system;
