#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent sequence components;
//!
//! Currently this crate does not contain its own `Sequence` type, as the types here are split from
//! the `object_model` crate, and the sequence type in that crate are specific to game object
//! definitions.

pub use sequence_model_derive::frame_component_data;

pub mod config;
pub mod loaded;
pub mod play;
