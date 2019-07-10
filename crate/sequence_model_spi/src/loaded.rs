//! Contains the types that represent processed configuration.

pub use self::{
    component_data_ext::ComponentDataExt, component_sequence::ComponentSequence,
    sequence_component_data::SequenceComponentData,
};

mod component_data_ext;
mod component_sequence;
mod sequence_component_data;
