use sequence_model::config::ControlTransitions;

use crate::config::{CharacterSequenceName, ControlTransitionRequirement};

/// Sequence ID to transition to when a `ControlAction` is pressed, held, or released.
pub type CharacterControlTransitions =
    ControlTransitions<CharacterSequenceName, Vec<ControlTransitionRequirement>>;
