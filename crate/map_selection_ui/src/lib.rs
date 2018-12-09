#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! UI to allow the user to select the map.

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate log;

#[macro_use]
extern crate strum_macros;

pub use crate::map_selection_ui_bundle::MapSelectionUiBundle;
pub(crate) use crate::{
    component::{MapSelectionWidget, WidgetState},
    system::{MapSelectionWidgetInputSystem, MapSelectionWidgetUiSystem},
};

mod component;
mod map_selection_ui_bundle;
mod system;
