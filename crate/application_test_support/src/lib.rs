#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Test harness to support testing of Autexousious applications.
//!
//! This builds on top of the `amethyst_test` crate by providing single calls to common
//! application setups necessary to test Autexousious applications.

pub use crate::{
    autexousious_application::AutexousiousApplication,
    queries::{AssetQueries, SequenceQueries},
    setup_function::SetupFunction,
};

pub mod prelude;

mod autexousious_application;
mod queries;
mod setup_function;
