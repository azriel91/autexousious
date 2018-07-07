#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where the game play takes place.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate character_selection;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
extern crate game_play_state;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;
extern crate object_play;

pub use game_play_bundle::GamePlayBundle;
pub(crate) use system::{
    CharacterInputUpdateSystem, CharacterKinematicsSystem, CharacterSequenceUpdateSystem,
    ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
};

mod game_play_bundle;
mod system;
