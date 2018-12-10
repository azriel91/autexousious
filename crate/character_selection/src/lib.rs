#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! State where character selection takes place.

pub use crate::{
    character_selection_bundle::CharacterSelectionBundle,
    character_selection_state::{
        CharacterSelectionState, CharacterSelectionStateBuilder, CharacterSelectionStateDelegate,
    },
    system::CharacterSelectionSystem,
};

mod character_selection_bundle;
mod character_selection_state;
mod system;
