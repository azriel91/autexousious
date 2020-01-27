#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! State where character selection takes place.

pub use crate::{
    character_selection_state::{
        CharacterSelectionState, CharacterSelectionStateBuilder, CharacterSelectionStateDelegate,
    },
    system::CharacterSelectionSystem,
};

mod character_selection_state;
mod system;
