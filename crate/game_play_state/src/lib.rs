#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where the game play takes place.
//!
//! This is split from the `game_play` crate as it allows the `application_test_support` crate to
//! depend on this crate and spawn objects for use by other crates. The `game_play` crate can then
//! depend on the `application_test_support` crate for testing its systems.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate character_selection;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;

pub use animation_runner::AnimationRunner;
pub(crate) use character_entity_spawner::CharacterEntitySpawner;
pub use game_play_state::GamePlayState;

mod animation_runner;
mod character_entity_spawner;
mod game_play_state;
