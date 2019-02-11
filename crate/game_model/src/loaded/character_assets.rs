use std::collections::BTreeMap;

use asset_model::config::AssetSlug;
use character_loading::CharacterPrefabHandle;

/// Map of `CharacterPrefab` handles, keyed by their `AssetSlug`.
pub type CharacterAssets = BTreeMap<AssetSlug, CharacterPrefabHandle>;
