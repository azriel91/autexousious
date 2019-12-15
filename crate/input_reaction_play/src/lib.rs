#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Logic for input reactions at runtime.

pub use crate::system::{ButtonInputReactionsTransitionSystem, InputReactionsTransitionSystem};

mod system;
