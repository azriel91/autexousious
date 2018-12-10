use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::SpriteSheetDefinition;

/// Configuration type for all sprite sheet definitions for an object.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, new)]
pub struct SpritesDefinition {
    /// Sprite sheet definitions in the sprites file.
    pub sheets: Vec<SpriteSheetDefinition>,
}
