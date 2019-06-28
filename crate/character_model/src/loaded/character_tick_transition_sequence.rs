use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use sequence_model::{component_sequence, config::TickTransition};
use typename_derive::TypeName;

use crate::config::CharacterSequenceId;

/// Sequence of `TickTransition<CharacterSequenceId>` values.
#[component_sequence(TickTransition<CharacterSequenceId>, component_owned = copy)]
pub struct CharacterTickTransitionSequence;

#[inline]
fn copy(
    tick_transition: &TickTransition<CharacterSequenceId>,
) -> TickTransition<CharacterSequenceId> {
    *tick_transition
}
