use amethyst::assets::Handle;
use sequence_model_derive::frame_component_data;

use crate::config::Body;

/// Sequence for volumes that can be hit.
#[frame_component_data(Handle<Body>)]
pub struct BodySequence;
