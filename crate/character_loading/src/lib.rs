#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Processes character configuration into the loaded character model.

pub use character_loading_bundle::CharacterLoadingBundle;
pub use prefab::{CharacterEntityAugmenter, CharacterPrefab, CharacterPrefabHandle};
pub use system_data::CharacterComponentStorages;

mod character_loading_bundle;
mod prefab;
mod system_data;
