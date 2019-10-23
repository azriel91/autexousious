use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Read, World},
    shred::{ResourceId, SystemData},
};
use background_model::loaded::AssetLayerPositions;
use derivative::Derivative;
use sequence_model::loaded::{AssetWaitSequenceHandles, WaitSequence};
use sprite_model::loaded::{AssetSpriteRenderSequenceHandles, SpriteRenderSequence};

/// Resources needed to spawn a map.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapSpawningResources<'s> {
    /// `EntitiesRes` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Read<'s, AssetWaitSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Read<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetLayerPositions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_layer_positions: Read<'s, AssetLayerPositions>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
}
