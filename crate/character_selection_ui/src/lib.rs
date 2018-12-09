#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

#[macro_use]
extern crate log;

pub use crate::character_selection_ui_bundle::CharacterSelectionUiBundle;
pub(crate) use crate::{
    component::{CharacterSelectionWidget, WidgetState},
    system::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetUiSystem},
};

mod character_selection_ui_bundle;
mod component;
mod system;
