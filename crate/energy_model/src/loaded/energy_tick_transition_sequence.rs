use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use sequence_model::{component_sequence, config::TickTransition};
use typename_derive::TypeName;

use crate::config::EnergySequenceId;

/// Sequence of `TickTransition<EnergySequenceId>` values.
#[component_sequence(TickTransition<EnergySequenceId>, component_owned = copy)]
pub struct EnergyTickTransitionSequence;

#[inline]
fn copy(
    tick_transition: &TickTransition<EnergySequenceId>,
) -> TickTransition<EnergySequenceId> {
    *tick_transition
}
