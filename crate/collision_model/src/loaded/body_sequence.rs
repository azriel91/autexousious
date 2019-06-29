use amethyst::assets::Handle;
use sequence_model_derive::component_sequence;

use crate::config::Body;

/// Sequence for volumes that can be hit.
#[component_sequence(Handle<Body>)]
pub struct BodySequence;
