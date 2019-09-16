#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loader::{CharacterLoader, CHARACTER_TRANSITIONS_DEFAULT},
    character_loader_params::CharacterLoaderParams,
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PROCESSOR},
    character_loading_status::CharacterLoadingStatus,
    cts_loader::CtsLoader,
    cts_loader_params::CtsLoaderParams,
};

mod character_loader;
mod character_loader_params;
mod character_loading_bundle;
mod character_loading_status;
mod cts_loader;
mod cts_loader_params;
