use amethyst::{assets::AssetStorage, ecs::world::EntitiesRes};

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `Obj`: Loaded form of the object, such as `Character`.
pub type ObjectSpawningResources<'res, Obj> = (&'res EntitiesRes, &'res AssetStorage<Obj>);
