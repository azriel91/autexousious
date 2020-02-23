#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `network_mode_selection` to be controlled by stdio.

pub use crate::{
    network_mode_selection_event_stdin_mapper::NetworkModeSelectionEventStdinMapper,
    network_mode_selection_stdio_bundle::NetworkModeSelectionStdioBundle,
};

mod network_mode_selection_event_stdin_mapper;
mod network_mode_selection_stdio_bundle;
