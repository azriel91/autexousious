#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Menu to allow the user to select game mode.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
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
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use character_selection::CharacterSelection;
pub use character_selection_bundle::CharacterSelectionBundle;
pub use character_selection_event::CharacterSelectionEvent;
pub use character_selection_state::CharacterSelectionState;
pub use character_selections::CharacterSelections;
pub(crate) use component::{CharacterSelectionWidget, WidgetState};
pub(crate) use system::{CharacterSelectionSystem, CharacterSelectionWidgetUiSystem};

mod character_selection;
mod character_selection_bundle;
mod character_selection_event;
mod character_selection_state;
mod character_selections;
mod component;
mod system;
