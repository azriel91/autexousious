//! Contains the types that represent processed configuration.

pub use self::{
    component_sequence::ComponentSequence,
    component_sequences::{ComponentSequences, ComponentSequencesHandle},
    control_transition::ControlTransition,
    control_transition_hold::ControlTransitionHold,
    control_transition_press::ControlTransitionPress,
    control_transitions::ControlTransitions,
    sequence_end_transition::SequenceEndTransition,
    sequence_end_transitions::SequenceEndTransitions,
    wait_sequence::WaitSequence,
};

mod component_sequence;
mod component_sequences;
mod control_transition;
mod control_transition_hold;
mod control_transition_press;
mod control_transitions;
mod sequence_end_transition;
mod sequence_end_transitions;
mod wait_sequence;
