#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state and systems for game play.
//!
//! Note that game entities are spawned in the `GameLoadingState` provided by the `game_loading`
//! crate.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
#[cfg(test)]
extern crate application_test_support;
extern crate character_selection;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_loading;
extern crate game_model;
#[cfg(test)]
extern crate loading;
#[macro_use]
extern crate log;
extern crate map_model;
extern crate map_selection;
#[cfg(test)]
extern crate object_loading;
extern crate object_model;
extern crate object_play;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use game_play_bundle::GamePlayBundle;
pub use game_play_state::GamePlayState;
pub(crate) use system::{
    CharacterGroundingSystem, CharacterInputUpdateSystem, CharacterKinematicsSystem,
    CharacterSequenceUpdateSystem, ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
};

mod game_play_bundle;
mod game_play_state;
mod system;
