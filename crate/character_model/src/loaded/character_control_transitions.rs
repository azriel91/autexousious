use sequence_model::loaded::ControlTransitions;

use crate::{config::CharacterSequenceId, loaded::CharacterControlTransition};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterControlTransitions =
    ControlTransitions<CharacterSequenceId, CharacterControlTransition>;
