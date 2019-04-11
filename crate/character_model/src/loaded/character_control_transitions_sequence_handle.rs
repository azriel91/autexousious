use sequence_model::loaded::ControlTransitionsSequenceHandle;

use crate::{config::CharacterSequenceId, loaded::CharacterControlTransition};

/// Handle to a `CharacterControlTransitionsSequence`
pub type CharacterControlTransitionsSequenceHandle =
    ControlTransitionsSequenceHandle<CharacterSequenceId, CharacterControlTransition>;
