#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent sprite configuration.
//!
//! These are used by other types that use sprites such as the various object types and map
//! layers.

#[cfg(test)]
extern crate toml;

pub mod config;
