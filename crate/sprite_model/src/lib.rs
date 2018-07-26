#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent sprite configuration.
//!
//! These are used by other types that use sprites such as the various object types and map
//! layers.
//!
//! Currently this crate only provides a `config` module as the `loaded` forms are provided by
//! Amethyst.

#[macro_use]
extern crate derive_new;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate toml;

pub mod config;
