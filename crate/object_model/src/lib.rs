#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent objects and their configuration.
//!
//! Object types are listed in the [`ObjectType`][obj_type] enum.
//!
//! [obj_type]: enum.ObjectType.html




#[macro_use]
extern crate derivative;


#[macro_use]
extern crate derive_new;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate serde_derive;




use strum;

#[cfg(test)]
extern crate toml;

pub use crate::object_type::ObjectType;

pub mod config;
pub mod entity;
pub mod loaded;
mod object_type;
