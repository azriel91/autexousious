use amethyst::{
    assets::AssetStorage,
    ecs::{world::Entities, Read},
};
use object_model::loaded::Object;

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `ObTy`: Loaded form of the object, such as `Character`.
/// * `SeqId`: Sequence ID of the object, such as `CharacterSequenceId`.
pub type ObjectSpawningResources<'res, ObTy, SeqId> = (
    Entities<'res>,
    Read<'res, AssetStorage<ObTy>>,
    Read<'res, AssetStorage<Object<SeqId>>>,
);
