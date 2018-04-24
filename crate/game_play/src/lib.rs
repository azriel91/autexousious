#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the `State` where the game play takes place.

extern crate amethyst;
extern crate amethyst_animation;
extern crate application_ui;
extern crate game_model;
#[macro_use]
extern crate log;

pub use state::State;

mod state;
