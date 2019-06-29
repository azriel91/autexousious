use amethyst::renderer::SpriteRender;
use sequence_model_derive::component_sequence;

/// Sequence for sprites to draw.
///
/// Loaded from a sequence of `SpriteRef`s.
#[component_sequence(SpriteRender)]
pub struct SpriteRenderSequence;
