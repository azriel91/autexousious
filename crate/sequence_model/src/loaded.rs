//! Contains the types that represent processed configuration.

pub use self::{
    component_sequence::ComponentSequence,
    component_sequences::{ComponentSequences, ComponentSequencesHandle},
    wait_sequence::WaitSequence,
};

mod component_sequence;
mod component_sequences;
mod wait_sequence;
