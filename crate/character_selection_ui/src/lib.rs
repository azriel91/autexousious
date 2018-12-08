#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate log;

#[macro_use]
extern crate strum_macros;

use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::character_selection_ui_bundle::CharacterSelectionUiBundle;
pub(crate) use crate::{
    component::{CharacterSelectionWidget, WidgetState},
    system::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetUiSystem},
};

mod character_selection_ui_bundle;
mod component;
mod system;
