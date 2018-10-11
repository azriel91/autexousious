#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to represent collidable objects.

#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate shape_model;
#[cfg(test)]
extern crate toml;

pub mod config;
