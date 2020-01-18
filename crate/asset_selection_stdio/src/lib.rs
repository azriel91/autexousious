#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Extension to enable `character_selection` to be controlled by stdio.

pub use crate::{
    asset_selection_event_stdin_mapper::AssetSelectionEventStdinMapper,
    asset_selection_stdio_bundle::AssetSelectionStdioBundle,
};

mod asset_selection_event_stdin_mapper;
mod asset_selection_stdio_bundle;
