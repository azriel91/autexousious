//! Contains the types that represent the configuration on disk.

pub use self::{
    game_object_definition::GameObjectDefinition, object_asset_data::ObjectAssetData,
    object_definition::ObjectDefinition, object_frame::ObjectFrame,
    object_sequence::ObjectSequence,
};

mod game_object_definition;
mod object_asset_data;
mod object_definition;
mod object_frame;
mod object_sequence;
