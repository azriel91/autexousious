#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `game_play` to be controlled by stdio.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate application_event;
#[cfg(test)]
extern crate assets_test;
#[macro_use]
extern crate derive_new;
extern crate game_play_model;
extern crate stdio_spi;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use crate::{
    game_play_event_args::GamePlayEventArgs,
    game_play_event_stdin_mapper::GamePlayEventStdinMapper,
    game_play_stdio_bundle::GamePlayStdioBundle,
};

mod game_play_event_args;
mod game_play_event_stdin_mapper;
mod game_play_stdio_bundle;
