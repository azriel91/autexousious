use amethyst::renderer::SpriteRender;
use sequence_model_derive::frame_component_data;

/// Sequence for sprites to draw.
///
/// Loaded from a sequence of `SpriteRef`s.
#[frame_component_data(SpriteRender)]
pub struct SpriteRenderSequence;
