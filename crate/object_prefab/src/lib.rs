#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for game objects.

pub use crate::{
    object_entity_augmenter::ObjectEntityAugmenter,
    system_data::{ObjectComponentStorages, ObjectSpawningResources},
};

mod object_entity_augmenter;
mod system_data;
