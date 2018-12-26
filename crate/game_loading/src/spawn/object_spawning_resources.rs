use amethyst::{
    assets::{Asset, AssetStorage, Handle},
    ecs::{Entities, Read, WriteStorage},
};
use derivative::Derivative;
use object_model::{
    config::object::SequenceId,
    loaded::{Object, ObjectHandle},
};
use shred_derive::SystemData;

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `ObTy`: Loaded form of the object, such as `Character`.
/// * `SeqId`: Sequence ID of the object, such as `CharacterSequenceId`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSpawningResources<'s, ObTy, SeqId>
where
    ObTy: Asset,
    SeqId: SequenceId + 'static,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ObjectHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub object_handles: WriteStorage<'s, ObjectHandle<SeqId>>,
    /// `Object` loaded assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'s, AssetStorage<Object<SeqId>>>,
    /// `Handle<ObTy>` component storage.
    #[derivative(Debug = "ignore")]
    pub ob_ty_handles: WriteStorage<'s, Handle<ObTy>>,
    /// Object type loaded assets, such as `Character`.
    #[derivative(Debug = "ignore")]
    pub ob_ty_assets: Read<'s, AssetStorage<ObTy>>,
}
