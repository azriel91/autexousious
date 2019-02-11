use amethyst::assets::{Handle, Prefab};

use crate::CharacterPrefab;

/// Handle to a `CharacterPrefab`.
pub type CharacterPrefabHandle = Handle<Prefab<CharacterPrefab>>;
