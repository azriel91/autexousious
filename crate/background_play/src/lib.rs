#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides background behaviour logic.

pub use self::{
    sprite_sequence_component_storages::SpriteSequenceComponentStorages,
    sprite_sequence_entity_spawner::SpriteSequenceEntitySpawner,
    sprite_sequence_spawning_resources::SpriteSequenceSpawningResources,
};

mod sprite_sequence_component_storages;
mod sprite_sequence_entity_spawner;
mod sprite_sequence_spawning_resources;
