#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `network_join` to be controlled by stdio.

pub use crate::{
    network_join_event_stdin_mapper::NetworkJoinEventStdinMapper,
    network_join_stdio_bundle::NetworkJoinStdioBundle,
};

mod network_join_event_stdin_mapper;
mod network_join_stdio_bundle;
