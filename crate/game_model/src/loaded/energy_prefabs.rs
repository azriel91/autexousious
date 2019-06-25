use energy_prefab::EnergyPrefab;

use crate::loaded::GameObjectPrefabs;

/// Map of `EnergyPrefab` handles, keyed by their `AssetSlug`.
pub type EnergyPrefabs = GameObjectPrefabs<EnergyPrefab>;
