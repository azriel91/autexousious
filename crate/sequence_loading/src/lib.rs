#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Processes sequence configuration into the loaded model.

pub use crate::{
    sequence_end_transition_mapper::SequenceEndTransitionMapper,
    sequence_end_transitions_loader::SequenceEndTransitionsLoader,
    sequence_id_mapper::SequenceIdMapper, sequence_loading_bundle::SequenceLoadingBundle,
    wait_sequence_handles_loader::WaitSequenceHandlesLoader,
    wait_sequence_loader::WaitSequenceLoader,
};

mod sequence_end_transition_mapper;
mod sequence_end_transitions_loader;
mod sequence_id_mapper;
mod sequence_loading_bundle;
mod wait_sequence_handles_loader;
mod wait_sequence_loader;
