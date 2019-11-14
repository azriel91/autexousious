use amethyst::{
    assets::AssetStorage,
    ecs::{Entities, Read, World},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use kinematic_model::loaded::AssetPositionInits;
use sequence_model::loaded::{AssetSequenceEndTransitions, AssetWaitSequenceHandles, WaitSequence};
use sprite_model::loaded::{AssetSpriteRenderSequenceHandles, SpriteRenderSequence};
use ui_label_model::loaded::AssetUiSpriteLabels;

/// Resources needed to spawn a map.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiSpriteLabelSpawningResources<'s> {
    /// `EntitiesRes` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `AssetUiSpriteLabels` resource.
    #[derivative(Debug = "ignore")]
    pub asset_ui_sprite_labels: Read<'s, AssetUiSpriteLabels>,
    /// `AssetWaitSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_wait_sequence_handles: Read<'s, AssetWaitSequenceHandles>,
    /// `AssetSpriteRenderSequenceHandles` resource.
    #[derivative(Debug = "ignore")]
    pub asset_sprite_render_sequence_handles: Read<'s, AssetSpriteRenderSequenceHandles>,
    /// `AssetPositionInits` resource.
    #[derivative(Debug = "ignore")]
    pub asset_position_inits: Read<'s, AssetPositionInits>,
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
