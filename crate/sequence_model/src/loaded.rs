//! Contains the types that represent processed configuration.

pub use self::{
    control_transition::ControlTransition,
    control_transition_default::ControlTransitionDefault,
    control_transition_hold::ControlTransitionHold,
    control_transition_like::ControlTransitionLike,
    control_transition_press::ControlTransitionPress,
    control_transition_release::ControlTransitionRelease,
    control_transitions::ControlTransitions,
    sequence_end_transitions::SequenceEndTransitions,
    wait_sequence::{WaitSequence, WaitSequenceHandle},
    wait_sequence_handles::WaitSequenceHandles,
};

mod control_transition;
mod control_transition_default;
mod control_transition_hold;
mod control_transition_like;
mod control_transition_press;
mod control_transition_release;
mod control_transitions;
mod sequence_end_transitions;
mod wait_sequence;
mod wait_sequence_handles;
