#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the prefab types and processing logic for game objects.

pub use crate::{
    game_object_prefab::GameObjectPrefab,
    object_entity_augmenter::ObjectEntityAugmenter,
    object_prefab::ObjectPrefab,
    object_prefab_error::ObjectPrefabError,
    system_data::{ObjectComponentStorages, ObjectSpawningResources},
};

mod game_object_prefab;
mod object_entity_augmenter;
mod object_prefab;
mod object_prefab_error;
mod system_data;
