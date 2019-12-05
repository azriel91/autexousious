use input_reaction_model::config::InputReactions;

use crate::config::{CharacterIrr, CharacterSequenceName};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterInputReactions = InputReactions<CharacterSequenceName, CharacterIrr>;
