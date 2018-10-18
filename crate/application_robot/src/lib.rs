#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! This crate provides types to support automation of operations in an Amethyst application.
//!
//! One of the main use cases is automated testing. The types allow input to the application, which
//! can control it as a replacement for device input (e.g. keyboard, mouse).

extern crate amethyst;
extern crate application_input;
#[cfg(test)]
extern crate debug_util_amethyst;
#[macro_use]
extern crate derivative;
extern crate itertools;

pub use state::RobotState;

pub mod state;
