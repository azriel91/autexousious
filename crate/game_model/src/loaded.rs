//! Loaded form of game object model and map types.

pub use self::{
    character_prefabs::CharacterPrefabs, energy_prefabs::EnergyPrefabs,
    game_object_prefabs::GameObjectPrefabs, map_prefabs::MapPrefabs,
};

mod character_prefabs;
mod energy_prefabs;
mod game_object_prefabs;
mod map_prefabs;
