#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during character selection.

#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;
extern crate game_input;
extern crate game_model;
extern crate object_model;
extern crate strum;
#[macro_use]
extern crate strum_macros;

pub use character_selection::CharacterSelection;
pub use character_selection_event::CharacterSelectionEvent;
pub use character_selections::CharacterSelections;
pub use character_selections_status::CharacterSelectionsStatus;

mod character_selection;
mod character_selection_event;
mod character_selections;
mod character_selections_status;
