//! Configuration types for objects.

pub use self::{
    character::{CharacterDefinition, CharacterSequenceId},
    object_definition::ObjectDefinition,
    sequence::{ObjectFrame, Sequence, SequenceId},
};

mod character;
mod object_definition;
mod sequence;
