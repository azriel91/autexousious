use character_prefab::CharacterPrefab;

use crate::loaded::GameObjectPrefabs;

/// Map of `CharacterPrefab` handles, keyed by their `AssetSlug`.
pub type CharacterPrefabs = GameObjectPrefabs<CharacterPrefab>;
