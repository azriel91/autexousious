use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::world::EntitiesRes,
};

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `Obj`: Loaded form of the object, such as `Character`.
pub type ObjectSpawningResources<'s, Obj> =
    (&'s EntitiesRes, &'s Vec<Handle<Obj>>, &'s AssetStorage<Obj>);
