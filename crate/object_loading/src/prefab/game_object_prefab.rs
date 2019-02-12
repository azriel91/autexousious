use amethyst::assets::PrefabData;
use object_model::loaded::GameObject;

use crate::prefab::object_prefab::ObjectPrefab;

/// Associates a `PrefabData` with a `GameObject`.
pub trait GameObjectPrefab<'s>: PrefabData<'s> {
    /// The `GameObject` this `PrefabData` is for.
    type GameObject: GameObject;

    /// Returns a new instance of this `GameObjectPrefab`.
    fn new(object_prefab: ObjectPrefab<Self::GameObject>) -> Self;
}
