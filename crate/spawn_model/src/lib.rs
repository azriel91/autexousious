#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent spawn configuration.
//!
//! These are used by other types that use spawns such as the various object types and map
//! layers.

#[cfg(test)]
extern crate toml;

pub mod config;
pub mod loaded;
