use amethyst::assets::PrefabData;
use object_model::{config::ObjectAssetData, loaded::GameObject};

/// Associates a `PrefabData` with a `GameObject`.
pub trait GameObjectPrefab<'s>: PrefabData<'s> {
    /// The `GameObject` this `PrefabData` is for.
    type GameObject: GameObject;

    /// Returns a new instance of this `GameObjectPrefab`.
    fn new(
        object_asset_data: ObjectAssetData<<Self::GameObject as GameObject>::Definition>,
    ) -> Self;
}
