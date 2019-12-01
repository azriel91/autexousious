//! Contains the types that represent processed configuration.

pub use self::{
    action_hold::ActionHold,
    action_press::ActionPress,
    action_release::ActionRelease,
    asset_sequence_id_mappings::AssetSequenceIdMappings,
    axis_transition::AxisTransition,
    control_transition_like::ControlTransitionLike,
    fallback_transition::FallbackTransition,
    input_reaction::InputReaction,
    input_reactions::InputReactions,
    sequence_end_transition::SequenceEndTransition,
    sequence_end_transitions::SequenceEndTransitions,
    sequence_id::SequenceId,
    sequence_id_mappings::SequenceIdMappings,
    wait_sequence::{WaitSequence, WaitSequenceHandle},
    wait_sequence_handles::WaitSequenceHandles,
};

mod action_hold;
mod action_press;
mod action_release;
mod asset_sequence_id_mappings;
mod axis_transition;
mod control_transition_like;
mod fallback_transition;
mod input_reaction;
mod input_reactions;
mod sequence_end_transition;
mod sequence_end_transitions;
mod sequence_id;
mod sequence_id_mappings;
mod wait_sequence;
mod wait_sequence_handles;
