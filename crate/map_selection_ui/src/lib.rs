#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! UI to allow the user to select the map.

pub use crate::map_selection_ui_bundle::MapSelectionUiBundle;
pub(crate) use crate::{
    component::{MapSelectionWidget, WidgetState},
    system::{MapSelectionWidgetInputSystem, MapSelectionWidgetUiSystem},
};

mod component;
mod map_selection_ui_bundle;
mod system;
