use amethyst::{
    assets::{Asset, AssetStorage},
    ecs::{world::Entities, Read},
};
use derivative::Derivative;
use object_model::{config::object::SequenceId, loaded::Object};
use shred_derive::SystemData;

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `ObTy`: Loaded form of the object, such as `Character`.
/// * `SeqId`: Sequence ID of the object, such as `CharacterSequenceId`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct ObjectSpawningResources<'res, ObTy, SeqId>
where
    ObTy: Asset,
    SeqId: SequenceId + 'static,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'res>,
    /// Object type loaded assets, such as `Character`.
    #[derivative(Debug = "ignore")]
    pub ob_ty_assets: Read<'res, AssetStorage<ObTy>>,
    /// `Object` loaded assets.
    #[derivative(Debug = "ignore")]
    pub object_assets: Read<'res, AssetStorage<Object<SeqId>>>,
}
