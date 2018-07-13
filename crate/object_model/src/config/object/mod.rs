//! Configuration types for objects.

pub use self::character::{CharacterDefinition, CharacterSequenceId};
pub use self::object_definition::ObjectDefinition;
pub use self::sequence::{Frame, Sequence, SequenceId, SequenceState};

mod character;
mod object_definition;
mod sequence;
