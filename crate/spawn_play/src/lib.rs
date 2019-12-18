#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides spawn logic during game play.

pub use crate::{
    game_object_spawner::GameObjectSpawner,
    system::{SpawnGameObjectRectifySystem, SpawnGameObjectSystem},
    system_data::SpawnGameObjectResources,
};

mod game_object_spawner;
mod system;
mod system_data;
