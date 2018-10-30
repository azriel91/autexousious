#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness to support testing of Autexousious applications.
//!
//! This builds on top of the `amethyst_test_support` crate by providing single calls to common
//! application setups necessary to test Autexousious applications.

extern crate amethyst;
extern crate amethyst_test_support;
extern crate application_event;
extern crate asset_loading;
extern crate assets_test;
extern crate character_selection;
extern crate character_selection_model;
extern crate collision_loading;
extern crate collision_model;
extern crate game_input;
extern crate game_loading;
extern crate game_model;
extern crate loading;
extern crate map_loading;
extern crate map_model;
extern crate map_selection;
extern crate map_selection_model;
extern crate object_loading;
extern crate object_model;
#[cfg(test)]
extern crate strum;

pub use autexousious_application::AutexousiousApplication;
pub use setup_function::SetupFunction;

mod autexousious_application;
pub mod prelude;
mod setup_function;
