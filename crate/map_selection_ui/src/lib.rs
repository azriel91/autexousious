#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! UI to allow the user to select the map.

pub use crate::{
    component::MapSelectionWidgetState,
    map_selection_ui_bundle::MapSelectionUiBundle,
    system::{
        MapSelectionSfxSystem, MapSelectionWidgetInputSystem, MapSelectionWidgetInputSystemData,
        MapSelectionWidgetUiSystem,
    },
};

mod component;
mod map_selection_ui_bundle;
mod system;
