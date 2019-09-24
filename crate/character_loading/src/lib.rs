#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PROCESSOR},
    character_transitions_default::CHARACTER_TRANSITIONS_DEFAULT,
    cts_loader::CtsLoader,
    cts_loader_params::CtsLoaderParams,
};

mod character_loading_bundle;
mod character_transitions_default;
mod cts_loader;
mod cts_loader_params;
