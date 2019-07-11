use amethyst::assets::Handle;
use sequence_model_derive::frame_component_data;

use crate::config::Interactions;

/// Sequence for interactions.
#[frame_component_data(Handle<Interactions>)]
pub struct InteractionsSequence;
