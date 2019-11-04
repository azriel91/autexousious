#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides background behaviour logic.

pub use self::{
    background_layer_component_storages::BackgroundLayerComponentStorages,
    background_layer_entity_spawner::BackgroundLayerEntitySpawner,
    background_layer_spawning_resources::BackgroundLayerSpawningResources,
};

mod background_layer_component_storages;
mod background_layer_entity_spawner;
mod background_layer_spawning_resources;
