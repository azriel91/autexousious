//! Loaded form of game object model and map types.

pub use self::{
    character_assets::CharacterAssets, game_object_prefabs::GameObjectPrefabs,
    map_assets::MapAssets,
};

mod character_assets;
mod game_object_prefabs;
mod map_assets;
