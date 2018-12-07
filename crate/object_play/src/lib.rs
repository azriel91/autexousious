#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Behaviour logic for object types.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate object_model;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::character::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};

mod character;
