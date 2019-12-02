use sequence_model::config::InputReactions;

use crate::config::{CharacterSequenceName, InputReactionRequirement};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterInputReactions =
    InputReactions<CharacterSequenceName, Vec<InputReactionRequirement>>;
