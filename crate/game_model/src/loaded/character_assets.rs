use std::collections::BTreeMap;

use object_model::loaded::CharacterHandle;

use crate::config::AssetSlug;

/// Map of `Character` asset handles, keyed by their `AssetSlug`.
pub type CharacterAssets = BTreeMap<AssetSlug, CharacterHandle>;
