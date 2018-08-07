use amethyst::{
    assets::{AssetStorage},
    ecs::world::EntitiesRes,
};
use map_model::loaded::Map;

/// Resources needed to spawn a map.
pub type MapSpawningResources<'res> = (
    &'res EntitiesRes,
    &'res AssetStorage<Map>,
);
