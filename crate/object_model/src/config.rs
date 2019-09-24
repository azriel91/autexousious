//! Contains the types that represent the configuration on disk.

pub use self::{
    game_object_frame::GameObjectFrame, game_object_sequence::GameObjectSequence, mass::Mass,
    object_definition::ObjectDefinition, object_frame::ObjectFrame,
    object_sequence::ObjectSequence,
};

mod game_object_frame;
mod game_object_sequence;
mod mass;
mod object_definition;
mod object_frame;
mod object_sequence;
