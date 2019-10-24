use sequence_loading_spi::SequenceComponentDataLoader;
use sprite_model::loaded::{SpriteRenderSequenceHandle, SpriteRenderSequenceHandles};

/// Loads `SpriteRenderSequenceHandle`s from collections of sequences that contain `SpriteRender` values.
#[derive(Debug)]
pub struct SpriteRenderSequenceHandlesLoader;

impl SequenceComponentDataLoader for SpriteRenderSequenceHandlesLoader {
    type Component = SpriteRenderSequenceHandle;
    type ComponentData = SpriteRenderSequenceHandles;
}
