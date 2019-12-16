use input_reaction_model::loaded::InputReaction;

use crate::config::CharacterIrr;

/// `InputReaction` with character input reaction requirement.
pub type CharacterInputReaction = InputReaction<CharacterIrr>;
