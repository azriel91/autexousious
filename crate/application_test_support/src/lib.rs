#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness to support testing of Autexousious applications.
//!
//! This builds on top of the `amethyst_test` crate by providing single calls to common
//! application setups necessary to test Autexousious applications.






















pub use crate::autexousious_application::AutexousiousApplication;
pub use crate::setup_function::SetupFunction;

mod autexousious_application;
pub mod prelude;
mod setup_function;
