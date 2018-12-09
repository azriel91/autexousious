#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to represent collidable objects.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate toml;

pub mod animation;
pub mod config;
pub mod play;
