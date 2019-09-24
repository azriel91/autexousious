#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for characters.

pub use crate::{
    character_entity_augmenter::CharacterEntityAugmenter,
    system_data::{CharacterComponentStorages, CharacterSpawningResources},
};

mod character_entity_augmenter;
mod system_data;
