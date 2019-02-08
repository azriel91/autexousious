//! Contains the types that represent the configuration on disk.

pub use self::{
    game_object_definition::GameObjectDefinition, object::ObjectDefinition,
    object_asset_data::ObjectAssetData,
};

pub mod object;

mod game_object_definition;
mod object_asset_data;
