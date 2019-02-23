use amethyst::assets::Handle;
use object_model_spi::loaded::ComponentFrames;

use crate::config::Body;

/// Sequence for volumes that can be hit.
pub type BodySequence = ComponentFrames<Handle<Body>>;
