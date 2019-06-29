use amethyst::assets::Handle;
use sequence_model_derive::component_sequence;

use crate::config::Interactions;

/// Sequence for interactions.
#[component_sequence(Handle<Interactions>)]
pub struct InteractionsSequence;
