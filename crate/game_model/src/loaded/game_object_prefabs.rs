use std::collections::BTreeMap;

use amethyst::assets::{Handle, Prefab};
use asset_model::config::AssetSlug;

/// Map of `GameObjectPrefab` handles, keyed by their `AssetSlug`.
pub type GameObjectPrefabs<Pf> = BTreeMap<AssetSlug, Handle<Prefab<Pf>>>;
