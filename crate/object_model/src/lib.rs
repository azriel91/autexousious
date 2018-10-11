#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent objects and their configuration.
//!
//! Object types are listed in the [`ObjectType`][obj_type] enum.
//!
//! [obj_type]: enum.ObjectType.html

extern crate amethyst;
extern crate collision_model;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate shape_model;
extern crate sprite_loading;
extern crate sprite_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[cfg(test)]
extern crate toml;

pub use object_type::ObjectType;

pub mod config;
pub mod entity;
pub mod loaded;
mod object_type;
