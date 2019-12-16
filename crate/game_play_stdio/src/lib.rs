#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `game_play` to be controlled by stdio.

pub use crate::{
    game_play_event_stdin_mapper::GamePlayEventStdinMapper,
    game_play_stdio_bundle::GamePlayStdioBundle,
};

mod game_play_event_stdin_mapper;
mod game_play_stdio_bundle;
