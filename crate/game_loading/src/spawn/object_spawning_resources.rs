use amethyst::{
    assets::{Asset, AssetStorage, Handle},
    ecs::{Read, WriteStorage},
};
use derivative::Derivative;
use object_model::loaded::GameObject;
use shred_derive::SystemData;

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `O`: Loaded form of the object, such as `Character`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSpawningResources<'s, O>
where
    O: Asset + GameObject,
{
    /// `Handle<O::ObjectWrapper>` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: WriteStorage<'s, Handle<O::ObjectWrapper>>,
    /// `Object` loaded assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'s, AssetStorage<O::ObjectWrapper>>,
    /// `Handle<O>` component storage.
    #[derivative(Debug = "ignore")]
    pub ob_ty_handles: WriteStorage<'s, Handle<O>>,
    /// Object type loaded assets, such as `Character`.
    #[derivative(Debug = "ignore")]
    pub ob_ty_assets: Read<'s, AssetStorage<O>>,
}
