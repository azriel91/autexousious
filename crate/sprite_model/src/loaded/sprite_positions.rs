use sequence_model_derive::sequence_component_data;

use crate::config::SpritePosition;

/// Sequence of `SpritePosition`s.
#[sequence_component_data(SpritePosition)]
pub struct SpritePositions;
