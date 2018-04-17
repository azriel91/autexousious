#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used to specify object configuration.
//!
//! Configuration types used by most (if not all) objects:
//!
//! * [`SpriteSheetDefinition`][spritesheet]: Defines how sprites are laid out on sprite sheets.
//!
//! [spritesheet]: struct.SpriteSheetDefinition.html

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate toml;

pub use sprite::{SpriteOffset, SpriteSheetDefinition};

mod sprite;
