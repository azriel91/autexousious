use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use sequence_model::component_sequence;

use crate::loaded::CharacterControlTransitionsHandle;

/// Sequence of sequence transitions upon control input.
#[component_sequence(CharacterControlTransitionsHandle)]
pub struct CharacterControlTransitionsSequence;
