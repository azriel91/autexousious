use amethyst::renderer::SpriteRender;
use sequence_model_spi::loaded::ComponentFrames;

/// Sequence for sprites to draw.
///
/// Loaded from a sequence of `SpriteRef`s.
pub type SpriteRenderSequence = ComponentFrames<SpriteRender>;
