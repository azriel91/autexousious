#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Behaviour logic for object types.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::character::{
    CharacterSequenceUpdateComponents, CharacterSequenceUpdater, MirroredUpdater, RunCounterUpdater,
};

mod character;
