//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    game_object::GameObject,
    object::Object,
    object_wrapper::ObjectWrapper,
    sequence::{SequenceEndTransition, SequenceEndTransitions},
};

mod game_object;
mod object;
mod object_wrapper;
mod sequence;
