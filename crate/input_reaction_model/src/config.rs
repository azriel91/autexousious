//! Contains the types that represent the configuration on disk.

pub use self::{
    input_reaction::InputReaction, input_reaction_app_event::InputReactionAppEvent,
    input_reaction_app_events::InputReactionAppEvents,
    input_reaction_multiple::InputReactionMultiple, input_reaction_single::InputReactionSingle,
    input_reactions::InputReactions,
};

mod input_reaction;
mod input_reaction_app_event;
mod input_reaction_app_events;
mod input_reaction_multiple;
mod input_reaction_single;
mod input_reactions;
