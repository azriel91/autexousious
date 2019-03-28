#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Types used for controlling stdio behaviour.

pub use crate::{
    barrier::Barrier, stdin_command_barrier::StdinCommandBarrier,
    stdio_command_event::StdioCommandEvent,
};

mod barrier;
mod stdin_command_barrier;
mod stdio_command_event;
