#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `session_host` to be controlled by stdio.

pub use crate::{
    session_host_event_stdin_mapper::SessionHostEventStdinMapper,
    session_host_stdio_bundle::SessionHostStdioBundle,
};

mod session_host_event_stdin_mapper;
mod session_host_stdio_bundle;
