use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Read, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use map_model::loaded::Map;
use sequence_model::loaded::WaitSequence;
use sprite_model::loaded::SpriteRenderSequence;

/// Resources needed to spawn a map.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSpawningResources<'res> {
    /// `EntitiesRes` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'res>,
    /// `Map` assets.
    #[derivative(Debug = "ignore")]
    pub map_assets: Read<'res, AssetStorage<Map>>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'res, AssetStorage<WaitSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'res, AssetStorage<SpriteRenderSequence>>,
}
