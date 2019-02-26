#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes object configuration into the loaded object model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub use crate::{
    object::{ObjectLoader, ObjectLoaderParams},
    prefab::{GameObjectPrefab, ObjectEntityAugmenter, ObjectPrefab, ObjectPrefabError},
    system::ObjectDefinitionToWrapperProcessor,
    system_data::{ObjectAnimationStorages, ObjectComponentStorages, ObjectFrameComponentStorages},
};

mod object;
mod prefab;
mod system;
mod system_data;
