use character_loading::CharacterPrefab;

use crate::loaded::GameObjectPrefabs;

/// Map of `CharacterPrefab` handles, keyed by their `AssetSlug`.
pub type CharacterAssets = GameObjectPrefabs<CharacterPrefab>;
