#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to represent maps and their configuration.
//!
//! Currently this is barebones and just allows us to define the bounds of a map for a game.

extern crate amethyst;
#[macro_use]
extern crate derive_new;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate toml;

pub mod config;
pub mod loaded;
