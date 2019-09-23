#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for characters.

pub use crate::{
    character_entity_augmenter::CharacterEntityAugmenter,
    character_prefab::CharacterPrefab,
    character_prefab_bundle::CharacterPrefabBundle,
    character_prefab_handle::CharacterPrefabHandle,
    system_data::{CharacterComponentStorages, CharacterSpawningResources},
};

mod character_entity_augmenter;
mod character_prefab;
mod character_prefab_bundle;
mod character_prefab_handle;
mod system_data;
