#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides background behaviour logic.

pub use self::{
    layer_component_storages::LayerComponentStorages, layer_entity_spawner::LayerEntitySpawner,
    layer_spawning_resources::LayerSpawningResources,
};

mod layer_component_storages;
mod layer_entity_spawner;
mod layer_spawning_resources;
