#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides logic for UI widgets at runtime.

pub use crate::system::{UiActiveWidgetUpdateSystem, UiTextColourUpdateSystem};

mod system;
