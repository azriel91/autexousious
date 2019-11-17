#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used during character selection.

pub use crate::{
    character_selection::CharacterSelection, character_selection_entity::CharacterSelectionEntity,
    character_selection_event::CharacterSelectionEvent, character_selections::CharacterSelections,
    character_selections_status::CharacterSelectionsStatus,
};

mod character_selection;
mod character_selection_entity;
mod character_selection_event;
mod character_selections;
mod character_selections_status;
