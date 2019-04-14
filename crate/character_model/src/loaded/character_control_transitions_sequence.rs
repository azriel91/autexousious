use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use sequence_model::component_sequence;

use crate::loaded::CharacterControlTransitionsHandle;

/// Sequence of sequence transitions upon control input.
#[component_sequence(CharacterControlTransitionsHandle)]
pub struct CharacterControlTransitionsSequence;
