//! Contains the types that represent processed configuration.

pub use self::{
    source_handle_opt::SourceHandleOpt,
    source_sequence::{SourceSequence, SourceSequenceHandle},
    source_sequence_handles::SourceSequenceHandles,
};

mod source_handle_opt;
mod source_sequence;
mod source_sequence_handles;
