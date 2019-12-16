#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used for controlling stdio behaviour.

pub use crate::{
    state_barrier::StateBarrier, stdin_command_barrier::StdinCommandBarrier,
    stdio_command_event::StdioCommandEvent, stdio_command_event_args::StdioCommandEventArgs,
};

mod state_barrier;
mod stdin_command_barrier;
mod stdio_command_event;
mod stdio_command_event_args;
