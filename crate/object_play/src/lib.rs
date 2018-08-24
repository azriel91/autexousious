#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Behaviour logic for object types.

extern crate amethyst;
extern crate game_input;
extern crate object_model;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use character::CharacterSequenceUpdater;

mod character;
