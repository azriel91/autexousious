#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_input_reactions_default::CHARACTER_INPUT_REACTIONS_DEFAULT,
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PROCESSOR},
};

mod character_input_reactions_default;
mod character_loading_bundle;
