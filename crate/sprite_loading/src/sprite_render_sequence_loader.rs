use amethyst::renderer::SpriteRender;
use sprite_model::{loaded::SpriteRenderSequence};
use sequence_loading_spi::FrameComponentDataLoader;

/// Loads `SpriteRenderSequence`s from `Sequence` types whose `Frame`s contain a `SpriteRender` value.
#[derive(Debug)]
pub struct SpriteRenderSequenceLoader;

impl FrameComponentDataLoader for SpriteRenderSequenceLoader {
    type Component = SpriteRender;
    type ComponentData = SpriteRenderSequence;
}
