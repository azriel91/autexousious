use amethyst::assets::Handle;
use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use sequence_model_derive::component_sequence;
use typename_derive::TypeName;

use crate::config::Interactions;

/// Sequence for interactions.
#[component_sequence(Handle<Interactions>)]
pub struct InteractionsSequence;
