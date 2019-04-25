#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes object configuration into the loaded object model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    object::{ObjectLoader, ObjectLoaderParams},
    prefab::{GameObjectPrefab, ObjectEntityAugmenter, ObjectPrefab, ObjectPrefabError},
    system::ObjectDefinitionToWrapperProcessor,
    system_data::{FrameComponentStorages, ObjectComponentStorages},
};

mod object;
mod prefab;
mod system;
mod system_data;
