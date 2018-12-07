#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! State where character selection takes place.










#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;



#[macro_use]
extern crate log;



use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::character_selection_bundle::CharacterSelectionBundle;
pub use crate::character_selection_state::{
    CharacterSelectionState, CharacterSelectionStateBuilder, CharacterSelectionStateDelegate,
};
pub use crate::system::CharacterSelectionSystem;

mod character_selection_bundle;
mod character_selection_state;
mod system;
