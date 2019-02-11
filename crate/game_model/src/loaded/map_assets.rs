use std::collections::BTreeMap;

use asset_model::config::AssetSlug;
use map_model::loaded::MapHandle;

/// Map (collection) of `Map` asset handles, keyed by their `AssetSlug`.
pub type MapAssets = BTreeMap<AssetSlug, MapHandle>;
