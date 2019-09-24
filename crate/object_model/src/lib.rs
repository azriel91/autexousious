#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent objects and their configuration.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod config;
pub mod loaded;
pub mod play;
