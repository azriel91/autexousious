#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `game_mode_selection` to be controlled by stdio.

pub use crate::{
    game_mode_selection_event_stdin_mapper::GameModeSelectionEventStdinMapper,
    game_mode_selection_stdio_bundle::GameModeSelectionStdioBundle,
};

mod game_mode_selection_event_stdin_mapper;
mod game_mode_selection_stdio_bundle;
