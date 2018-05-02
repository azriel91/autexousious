#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where the game play takes place.

extern crate amethyst;
extern crate application_ui;
extern crate character_selection;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate object_model;

pub use game_play_state::GamePlayState;

mod game_play_state;
