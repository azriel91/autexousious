#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Types used during character selection.


#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;




#[macro_use]
extern crate strum_macros;

pub use crate::character_selection::CharacterSelection;
pub use crate::character_selection_entity::CharacterSelectionEntity;
pub use crate::character_selection_entity_id::CharacterSelectionEntityId;
pub use crate::character_selection_event::CharacterSelectionEvent;
pub use crate::character_selections::CharacterSelections;
pub use crate::character_selections_status::CharacterSelectionsStatus;

mod character_selection;
mod character_selection_entity;
mod character_selection_entity_id;
mod character_selection_event;
mod character_selections;
mod character_selections_status;
