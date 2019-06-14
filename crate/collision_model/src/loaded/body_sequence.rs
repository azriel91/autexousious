use amethyst::assets::Handle;
use asset_derive::Asset;
use derive_deref::{Deref, DerefMut};
use sequence_model_derive::component_sequence;
use typename_derive::TypeName;

use crate::config::Body;

/// Sequence for volumes that can be hit.
#[component_sequence(Handle<Body>)]
pub struct BodySequence;
