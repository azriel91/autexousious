use sequence_model::loaded::ControlTransitionsSequence;

use crate::{config::CharacterSequenceId, loaded::CharacterControlTransition};

/// Sequence of sequence transitions upon control input.
pub type CharacterControlTransitionsSequence =
    ControlTransitionsSequence<CharacterSequenceId, CharacterControlTransition>;
