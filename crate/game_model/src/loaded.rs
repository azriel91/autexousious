//! Loaded form of game object model and map types.

pub use self::{
    character_prefabs::CharacterPrefabs, game_object_prefabs::GameObjectPrefabs,
    map_prefabs::MapPrefabs,
};

mod character_prefabs;
mod game_object_prefabs;
mod map_prefabs;
