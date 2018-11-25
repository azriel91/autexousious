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
extern crate tracker;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use character::CharacterSequenceUpdater;
pub use object_play_bundle::ObjectPlayBundle;
pub use system::RunCounterUpdateSystem;

mod character;
mod object_play_bundle;
mod system;
