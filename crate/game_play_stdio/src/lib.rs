#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Extension to enable `game_play` to be controlled by stdio.





#[macro_use]
extern crate derive_new;


use structopt;
#[macro_use]
extern crate structopt_derive;
use typename;
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
