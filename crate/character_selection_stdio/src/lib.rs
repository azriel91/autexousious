#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `character_selection` to be controlled by stdio.

pub use crate::{
    character_selection_event_stdin_mapper::CharacterSelectionEventStdinMapper,
    character_selection_stdio_bundle::CharacterSelectionStdioBundle,
};

mod character_selection_event_stdin_mapper;
mod character_selection_stdio_bundle;
