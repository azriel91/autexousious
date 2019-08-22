//! Contains the types that represent processed configuration.

pub use self::{
    action_hold::ActionHold,
    action_press::ActionPress,
    action_release::ActionRelease,
    axis_transition::AxisTransition,
    control_transition::ControlTransition,
    control_transition_like::ControlTransitionLike,
    control_transitions::ControlTransitions,
    fallback_transition::FallbackTransition,
    sequence_end_transition::SequenceEndTransition,
    sequence_end_transitions::SequenceEndTransitions,
    sequence_id::SequenceId,
    wait_sequence::{WaitSequence, WaitSequenceHandle},
    wait_sequence_handles::WaitSequenceHandles,
};

mod action_hold;
mod action_press;
mod action_release;
mod axis_transition;
mod control_transition;
mod control_transition_like;
mod control_transitions;
mod fallback_transition;
mod sequence_end_transition;
mod sequence_end_transitions;
mod sequence_id;
mod wait_sequence;
mod wait_sequence_handles;
