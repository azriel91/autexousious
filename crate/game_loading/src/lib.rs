#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where loading of game entities takes place.
//!
//! This is split from the `game_play` crate as it allows the `application_test_support` crate to
//! depend on this crate and spawn objects for use by other crates. The `game_play` crate can then
//! depend on the `application_test_support` crate for testing its systems.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate character_selection;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_model;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate map_loading;
extern crate map_model;
extern crate map_selection;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;
#[cfg(test)]
extern crate typename;
#[cfg(test)]
#[macro_use]
extern crate typename_derive;

pub use animation_runner::AnimationRunner;
pub(crate) use character_entity_spawner::CharacterEntitySpawner;
pub use game_loading_state::GameLoadingState;
pub(crate) use map_layer_entity_spawner::MapLayerEntitySpawner;

mod animation_runner;
mod character_entity_spawner;
mod game_loading_state;
mod map_layer_entity_spawner;
