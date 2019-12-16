use input_reaction_model::loaded::{InputReactions, InputReactionsHandle};

use crate::loaded::CharacterInputReaction;

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterInputReactions = InputReactions<CharacterInputReaction>;

/// Handle to `CharacterInputReactions`.
pub type CharacterInputReactionsHandle = InputReactionsHandle<CharacterInputReaction>;
