#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to represent objects and their configuration.
//!
//! Object types are listed in the [`ObjectType`][obj_type] enum.
//!
//! Configuration types used by most (if not all) objects:
//!
//! * [`SpriteSheetDefinition`][spritesheet]: Defines how sprites are laid out on sprite sheets.
//!
//! [obj_type]: enum.ObjectType.html
//! [spritesheet]: struct.SpriteSheetDefinition.html

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate toml;

pub use object_type::ObjectType;
pub use sprite::{SpriteOffset, SpriteSheetDefinition};

mod object_type;
mod sprite;
