//! Contains the types that represent processed configuration.

pub use self::{
    action_hold::ActionHold,
    action_press::ActionPress,
    action_release::ActionRelease,
    control_transition::ControlTransition,
    control_transition_default::ControlTransitionDefault,
    control_transition_like::ControlTransitionLike,
    control_transitions::ControlTransitions,
    sequence_end_transitions::SequenceEndTransitions,
    wait_sequence::{WaitSequence, WaitSequenceHandle},
    wait_sequence_handles::WaitSequenceHandles,
};

mod action_hold;
mod action_press;
mod action_release;
mod control_transition;
mod control_transition_default;
mod control_transition_like;
mod control_transitions;
mod sequence_end_transitions;
mod wait_sequence;
mod wait_sequence_handles;
