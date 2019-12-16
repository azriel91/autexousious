//! Contains the types that represent the configuration on disk.

pub use sequence_model_core::config::SequenceName;

pub use self::{
    frame::Frame,
    sequence::Sequence,
    sequence_end_transition::SequenceEndTransition,
    sequence_name_string::SequenceNameString,
    sequences::Sequences,
    wait::{Wait, WAIT_DEFAULT},
};

mod frame;
mod sequence;
mod sequence_end_transition;
mod sequence_name_string;
mod sequences;
mod wait;
