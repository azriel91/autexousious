use std::collections::BTreeMap;

use map_model::loaded::MapHandle;

use config::AssetSlug;

/// Map (collection) of `Map` asset handles, keyed by their `AssetSlug`.
pub type MapAssets = BTreeMap<AssetSlug, MapHandle>;
