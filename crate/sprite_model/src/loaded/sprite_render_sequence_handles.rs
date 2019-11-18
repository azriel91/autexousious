use amethyst::ecs::{storage::DenseVecStorage, Component};
use asset_model::ItemComponent;
use sequence_model_derive::sequence_component_data;

use crate::loaded::SpriteRenderSequenceHandle;

/// Sequence of `SpriteRenderSequenceHandle`s.
#[sequence_component_data(SpriteRenderSequenceHandle)]
#[derive(ItemComponent)]
#[storage(DenseVecStorage)]
pub struct SpriteRenderSequenceHandles;
