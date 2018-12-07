#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
#[cfg(test)]
extern crate application_test_support;
extern crate application_ui;
#[cfg(test)]
extern crate assets_test;
extern crate character_selection_model;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
#[macro_use]
extern crate log;
extern crate object_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate tracker;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use crate::character_selection_ui_bundle::CharacterSelectionUiBundle;
pub(crate) use crate::component::{CharacterSelectionWidget, WidgetState};
pub(crate) use crate::system::{CharacterSelectionWidgetInputSystem, CharacterSelectionWidgetUiSystem};

mod character_selection_ui_bundle;
mod component;
mod system;
