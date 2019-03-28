#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used for controlling stdio behaviour.

pub use crate::{
    state_barrier::StateBarrier, stdin_command_barrier::StdinCommandBarrier,
    stdio_command_event::StdioCommandEvent,
};

mod state_barrier;
mod stdin_command_barrier;
mod stdio_command_event;
