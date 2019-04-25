#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent shapes.

pub use crate::{axis::Axis, volume::Volume};

mod axis;
mod volume;
