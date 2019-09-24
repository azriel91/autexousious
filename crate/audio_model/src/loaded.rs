//! Contains the types that represent processed configuration.

pub use self::{
    asset_source_sequence_handles::AssetSourceSequenceHandles,
    source_handle_opt::SourceHandleOpt,
    source_sequence::{SourceSequence, SourceSequenceHandle},
    source_sequence_handles::SourceSequenceHandles,
};

mod asset_source_sequence_handles;
mod source_handle_opt;
mod source_sequence;
mod source_sequence_handles;
