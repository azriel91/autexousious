//! Contains the types that represent processed configuration.

pub use self::{
    axis_transition::AxisTransition,
    fallback_transition::FallbackTransition,
    input_reaction::InputReaction,
    input_reactions::{InputReactions, InputReactionsHandle},
    input_reactions_sequence::{InputReactionsSequence, InputReactionsSequenceHandle},
    input_reactions_sequence_handles::InputReactionsSequenceHandles,
    reaction_effect::ReactionEffect,
    reaction_effect_data::ReactionEffectData,
};

mod axis_transition;
mod fallback_transition;
mod input_reaction;
mod input_reactions;
mod input_reactions_sequence;
mod input_reactions_sequence_handles;
mod reaction_effect;
mod reaction_effect_data;
