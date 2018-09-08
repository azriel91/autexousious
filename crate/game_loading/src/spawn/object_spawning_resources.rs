use std::collections::BTreeMap;

use amethyst::{
    assets::{AssetStorage, Handle},
    ecs::world::EntitiesRes,
};
use game_model::config::AssetSlug;

/// Resources needed to spawn a game object.
///
/// # Type Parameters:
///
/// * `Obj`: Loaded form of the object, such as `Character`.
pub type ObjectSpawningResources<'res, Obj> = (
    &'res EntitiesRes,
    &'res BTreeMap<AssetSlug, Handle<Obj>>,
    &'res AssetStorage<Obj>,
);
