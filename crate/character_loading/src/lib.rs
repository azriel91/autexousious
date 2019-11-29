#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PROCESSOR},
    character_transitions_default::CHARACTER_TRANSITIONS_DEFAULT,
    irs_loader::IrsLoader,
    irs_loader_params::IrsLoaderParams,
};

mod character_loading_bundle;
mod character_transitions_default;
mod irs_loader;
mod irs_loader_params;
