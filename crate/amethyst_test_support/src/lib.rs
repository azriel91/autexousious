#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Test harness support to cover testing of the following types:
//!
//! * `Bundle`
//! * `State`
//! * `System`
//! * Resource loading.
//! * Arbitrary types that `System`s use during processing.
//!
//! This crate also aims to minimize boilerplate for:
//!
//! * Setting up `GameData` and an `Application` with common bundles
//! * Mock `State`s:
//!     - Empty functionality (simply to feed into the `Application`)
//!     - `.update()` with `assertion_fn`
//!     - `.update()` with `setup_fn` and `assertion_fn` &mdash; e.g. loading resources needs an
//!       `.update()` call before the resource is actually loaded.

extern crate amethyst;
extern crate boxfnonce;
#[macro_use]
extern crate derivative;

pub use amethyst_application::AmethystApplication;
pub use empty_state::EmptyState;

mod amethyst_application;
mod empty_state;
pub mod prelude;
