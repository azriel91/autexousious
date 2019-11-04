use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Read, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use sequence_model::loaded::{AssetSequenceEndTransitions, AssetWaitSequenceHandles, WaitSequence};
use sprite_model::loaded::{
    AssetSpritePositions, AssetSpriteRenderSequenceHandles, SpriteRenderSequence,
};

/// Resources needed to spawn a map.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpriteSequenceSpawningResources<'s> {
    /// `EntitiesRes` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Read<'s, AssetWaitSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Read<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetSpritePositions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_positions: Read<'s, AssetSpritePositions>,
    /// `AssetSequenceEndTransitions` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sequence_end_transitions: Read<'s, AssetSequenceEndTransitions>,
    /// `WaitSequence` assets.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_assets: Read<'s, AssetStorage<WaitSequence>>,
    /// `SpriteRenderSequence` assets.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_assets: Read<'s, AssetStorage<SpriteRenderSequence>>,
}
