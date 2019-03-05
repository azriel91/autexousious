use amethyst::assets::Handle;
use sequence_model_spi::loaded::ComponentFrames;

use crate::config::Interactions;

/// Sequence for interactions.
pub type InteractionsSequence = ComponentFrames<Handle<Interactions>>;
