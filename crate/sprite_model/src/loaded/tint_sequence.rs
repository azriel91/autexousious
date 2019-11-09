use amethyst::renderer::resources::Tint;

use sequence_model::frame_component_data;

/// Sequence of `Tint` values.
#[frame_component_data(Tint, copy)]
pub struct TintSequence;
