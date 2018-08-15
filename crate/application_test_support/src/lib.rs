#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness to support testing of Autexousious applications.
//!
//! This builds on top of the `amethyst_test_support` crate by providing single calls to common
//! application setups necessary to test Autexousious applications.

extern crate amethyst;
extern crate amethyst_test_support;
extern crate application;
extern crate character_selection;
extern crate game_input;
extern crate loading;
extern crate map_loading;
#[cfg(test)]
extern crate map_model;
extern crate object_loading;
extern crate object_model;

pub use autexousious_application::AutexousiousApplication;

mod autexousious_application;
