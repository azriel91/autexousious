#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used to represent objects and their configuration.
//!
//! Object types are listed in the [`ObjectType`][obj_type] enum.
//!
//! [obj_type]: enum.ObjectType.html

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

// Re-export proc macro attribute.
pub use object_model_derive::game_object;

pub use crate::object_type::ObjectType;

pub mod config;
pub mod entity;
pub mod loaded;
mod object_type;
