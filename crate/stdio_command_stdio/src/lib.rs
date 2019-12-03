#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `stdio_input` to be controlled by stdio.

pub use crate::{
    stdio_command_event_stdin_mapper::StdioCommandEventStdinMapper,
    stdio_command_processing_system::StdioCommandProcessingSystem,
    stdio_command_stdio_bundle::StdioCommandStdioBundle,
};

mod stdio_command_event_stdin_mapper;
mod stdio_command_processing_system;
mod stdio_command_stdio_bundle;
