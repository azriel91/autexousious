use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SpriteSheetDefinition;

/// Configuration type for all sprite sheet definitions for an object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SpritesDefinition {
    /// Sprite sheet definitions in the sprites file.
    pub sheets: Vec<SpriteSheetDefinition>,
}

impl Asset for SpritesDefinition {
    const NAME: &'static str = concat!(module_path!(), "::", stringify!(SpritesDefinition));
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<SpritesDefinition> for Result<ProcessingState<SpritesDefinition>, Error> {
    fn from(
        sprites_definition: SpritesDefinition,
    ) -> Result<ProcessingState<SpritesDefinition>, Error> {
        Ok(ProcessingState::Loaded(sprites_definition))
    }
}
