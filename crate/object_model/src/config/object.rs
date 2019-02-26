//! Configuration types for objects.

pub use self::{
    object_definition::ObjectDefinition,
    sequence::{FrameIndex, ObjectFrame, Sequence, SequenceId, Wait},
};

mod object_definition;
mod sequence;
