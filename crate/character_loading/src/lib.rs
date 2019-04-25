#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loader::CharacterLoader,
    character_loader_params::CharacterLoaderParams,
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PREFAB_LOADER_SYSTEM},
    prefab::{CharacterEntityAugmenter, CharacterPrefab, CharacterPrefabHandle},
    system_data::CharacterComponentStorages,
};

mod character_loader;
mod character_loader_params;
mod character_loading_bundle;
mod prefab;
mod system_data;
