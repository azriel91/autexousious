//! Contains the types that represent processed configuration.

pub use self::{
    asset_sequence_id_mappings::AssetSequenceIdMappings,
    sequence_end_transition::SequenceEndTransition,
    sequence_end_transitions::SequenceEndTransitions,
    sequence_id::SequenceId,
    sequence_id_mappings::SequenceIdMappings,
    wait_sequence::{WaitSequence, WaitSequenceHandle},
    wait_sequence_handles::WaitSequenceHandles,
};

mod asset_sequence_id_mappings;
mod sequence_end_transition;
mod sequence_end_transitions;
mod sequence_id;
mod sequence_id_mappings;
mod wait_sequence;
mod wait_sequence_handles;
