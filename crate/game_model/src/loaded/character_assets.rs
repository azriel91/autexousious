use std::collections::BTreeMap;

use asset_model::config::AssetSlug;
use character_model::loaded::CharacterHandle;

/// Map of `Character` asset handles, keyed by their `AssetSlug`.
pub type CharacterAssets = BTreeMap<AssetSlug, CharacterHandle>;
