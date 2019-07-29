#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loader::{CharacterLoader, CHARACTER_TRANSITIONS_DEFAULT},
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PROCESSOR},
    character_loading_status::CharacterLoadingStatus,
    control_transitions_sequence_loader::ControlTransitionsSequenceLoader,
    control_transitions_sequence_loader_params::ControlTransitionsSequenceLoaderParams,
};

mod character_loader;
mod character_loading_bundle;
mod character_loading_status;
mod control_transitions_sequence_loader;
mod control_transitions_sequence_loader_params;
