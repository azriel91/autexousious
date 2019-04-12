use derive_deref::{Deref, DerefMut};
use derive_more::From;
use derive_new::new;
use sequence_model::component_sequence;

use crate::loaded::CharacterControlTransitions;

/// Sequence of sequence transitions upon control input.
#[component_sequence(CharacterControlTransitions)]
pub struct CharacterControlTransitionsSequence;
