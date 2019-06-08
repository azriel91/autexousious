use amethyst::assets::{Handle, PrefabData};
use object_model::{config::ObjectAssetData, loaded::GameObject};

use crate::ObjectPrefab;

/// Associates a `PrefabData` with a `GameObject`.
pub trait GameObjectPrefab<'s>: PrefabData<'s> {
    /// The `GameObject` this `PrefabData` is for.
    type GameObject: GameObject;

    /// Returns a new instance of this `GameObjectPrefab`.
    fn new(
        object_asset_data: ObjectAssetData<<Self::GameObject as GameObject>::Definition>,
    ) -> Self;

    /// Returns `Handle<Self::GameObject>` if it has been loaded. `None` otherwise.
    fn game_object_handle(&self) -> Option<Handle<Self::GameObject>>;

    /// Returns a reference to this `GameObjectPrefab`'s inner `ObjectPrefab`.
    fn object_prefab(&self) -> &ObjectPrefab<Self::GameObject>;
}
