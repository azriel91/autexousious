#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes character configuration into the loaded character model.

pub use crate::{
    character_loader::CharacterLoader,
    character_loading_bundle::{CharacterLoadingBundle, CHARACTER_PREFAB_LOADER_SYSTEM},
    prefab::{CharacterEntityAugmenter, CharacterPrefab, CharacterPrefabHandle},
    system_data::CharacterComponentStorages,
};

mod character_loader;
mod character_loading_bundle;
mod prefab;
mod system_data;
