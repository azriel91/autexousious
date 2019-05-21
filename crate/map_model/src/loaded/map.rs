use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::{
        storage::{DenseVecStorage, VecStorage},
        Component,
    },
    renderer::sprite::SpriteSheetHandle,
    Error,
};
use derive_new::new;
use sequence_model::loaded::ComponentSequencesHandle;

use crate::{config::MapDefinition, loaded::Margins};

/// Loaded representation of a `Map`.
#[derive(Clone, Debug, PartialEq, new)]
pub struct Map {
    /// Map configuration.
    pub definition: MapDefinition,
    /// Coordinates of the limits of the playable area.
    pub margins: Margins,
    /// Handle to the sprite sheets for layer entities.
    pub sprite_sheet_handles: Option<Vec<SpriteSheetHandle>>,
    /// Handles to sequences of components that each layer has.
    pub component_sequences_handles: Option<Vec<ComponentSequencesHandle>>,
}

impl Asset for Map {
    const NAME: &'static str = "map_model::loaded::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        Ok(ProcessingState::Loaded(map))
    }
}

/// Handle to a Map
pub type MapHandle = Handle<Map>;
