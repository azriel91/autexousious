//! Contains the types that represent the configuration on disk.

pub use sequence_model_core::config::SequenceName;

pub use self::{
    control_transition::ControlTransition,
    control_transition_multiple::ControlTransitionMultiple,
    control_transition_single::ControlTransitionSingle,
    frame::Frame,
    input_reactions::InputReactions,
    sequence::Sequence,
    sequence_end_transition::SequenceEndTransition,
    sequence_name_string::SequenceNameString,
    sequences::Sequences,
    wait::{Wait, WAIT_DEFAULT},
};

mod control_transition;
mod control_transition_multiple;
mod control_transition_single;
mod frame;
mod input_reactions;
mod sequence;
mod sequence_end_transition;
mod sequence_name_string;
mod sequences;
mod wait;
