#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent objects and their configuration.
//!
//! Object types are listed in the [`ObjectType`][obj_type] enum.
//!
//! [obj_type]: enum.ObjectType.html

extern crate amethyst;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sprite_loading;
extern crate sprite_model;
#[cfg(test)]
extern crate toml;

pub use object_type::ObjectType;

pub mod config;
pub mod entity;
pub mod loaded;
mod object_type;
