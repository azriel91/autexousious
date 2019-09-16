use sequence_model_derive::frame_component_data;

use crate::loaded::CharacterControlTransitionsHandle;

/// Sequence of sequence transitions upon control input.
///
/// CTS is short for control transitions sequence.
#[frame_component_data(CharacterControlTransitionsHandle)]
pub struct CharacterCts;
