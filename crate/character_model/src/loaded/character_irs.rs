use input_reaction_model::loaded::{InputReactionsSequence, InputReactionsSequenceHandle};

use crate::loaded::CharacterInputReaction;

/// Sequence of input reactions.
pub type CharacterIrs = InputReactionsSequence<CharacterInputReaction>;

/// Handle to a `CharacterIrs`.
pub type CharacterIrsHandle = InputReactionsSequenceHandle<CharacterInputReaction>;
