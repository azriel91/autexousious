#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where the game play takes place.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(test)]
#[macro_use]
extern crate application;
extern crate character_selection;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;
extern crate object_play;

pub(crate) use animation_runner::AnimationRunner;
pub(crate) use character_entity_spawner::CharacterEntitySpawner;
pub(crate) use character_input_update_system::CharacterInputUpdateSystem;
pub use game_play_bundle::GamePlayBundle;
pub use game_play_state::GamePlayState;
pub(crate) use system::ObjectTransformUpdateSystem;

mod animation_runner;
mod character_entity_spawner;
mod character_input_update_system;
mod game_play_bundle;
mod game_play_state;
mod system;
