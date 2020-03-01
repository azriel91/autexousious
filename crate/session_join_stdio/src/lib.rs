#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `session_join` to be controlled by stdio.

pub use crate::{
    session_join_event_stdin_mapper::SessionJoinEventStdinMapper,
    session_join_stdio_bundle::SessionJoinStdioBundle,
};

mod session_join_event_stdin_mapper;
mod session_join_stdio_bundle;
