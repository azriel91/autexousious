use amethyst::{
    assets::AssetStorage,
    ecs::{world::Entities, Read},
};

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `Obj`: Loaded form of the object, such as `Character`.
pub type ObjectSpawningResources<'res, Obj> = (Entities<'res>, Read<'res, AssetStorage<Obj>>);
