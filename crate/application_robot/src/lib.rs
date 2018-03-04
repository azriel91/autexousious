#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! This crate provides types to support automation of operations in an Amethyst application.
//!
//! One of the main use cases is automated testing. The types allow input to the application, which
//! can control it as a replacement for device input (e.g. keyboard, mouse).

extern crate amethyst;
extern crate application_input;
#[cfg(test)]
extern crate enigo;
#[cfg(test)]
extern crate winit;

pub use state::State;

mod state;
