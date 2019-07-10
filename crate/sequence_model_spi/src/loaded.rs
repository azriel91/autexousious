//! Contains the types that represent processed configuration.

pub use self::{
    component_sequence::ComponentSequence, component_sequence_ext::ComponentSequenceExt,
    sequence_component_data::SequenceComponentData,
};

mod component_sequence;
mod component_sequence_ext;
mod sequence_component_data;
