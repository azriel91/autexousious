use sequence_model_derive::component_sequence;

use crate::loaded::CharacterControlTransitionsHandle;

/// Sequence of sequence transitions upon control input.
#[component_sequence(CharacterControlTransitionsHandle)]
pub struct CharacterControlTransitionsSequence;
