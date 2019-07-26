use sequence_model::config::ControlTransitions;

use crate::config::{CharacterSequenceId, ControlTransitionRequirement};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterControlTransitions =
    ControlTransitions<CharacterSequenceId, Vec<ControlTransitionRequirement>>;
