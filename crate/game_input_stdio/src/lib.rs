#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `game_input` to be controlled by stdio.

pub use crate::{
    control_input_event_stdin_mapper::ControlInputEventStdinMapper,
    game_input_stdio_bundle::GameInputStdioBundle, game_input_stdio_error::GameInputStdioError,
};

mod control_input_event_stdin_mapper;
mod game_input_stdio_bundle;
mod game_input_stdio_error;
