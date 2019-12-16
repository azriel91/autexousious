#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `map_selection` to be controlled by stdio.

pub use crate::{
    map_selection_event_stdin_mapper::{
        MapSelectionEventStdinMapper, MapSelectionEventStdinMapperSystemData,
    },
    map_selection_stdio_bundle::MapSelectionStdioBundle,
};

mod map_selection_event_stdin_mapper;
mod map_selection_stdio_bundle;
