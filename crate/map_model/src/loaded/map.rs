use amethyst::{
    ecs::{storage::DenseVecStorage, Component},
    renderer::sprite::SpriteSheetHandle,
};
use asset_derive::Asset;
use derive_new::new;
use sequence_model::loaded::WaitSequenceHandle;
use sprite_model::loaded::SpriteRenderSequenceHandle;

use crate::{config::MapDefinition, loaded::Margins};

/// Loaded representation of a `Map`.
#[derive(Asset, Clone, Debug, PartialEq, new)]
pub struct Map {
    /// Map configuration.
    pub definition: MapDefinition,
    /// Coordinates of the limits of the playable area.
    pub margins: Margins,
    /// Handle to the sprite sheets for layer entities.
    pub sprite_sheet_handles: Vec<SpriteSheetHandle>,
    /// Handles to wait sequences that each layer has.
    pub wait_sequence_handles: Vec<WaitSequenceHandle>,
    /// Handles to wait sequences that each layer has.
    pub sprite_render_sequence_handles: Vec<SpriteRenderSequenceHandle>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}
