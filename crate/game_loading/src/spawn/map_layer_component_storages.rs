use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::{transparent::Transparent, SpriteRender},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use map_model::config::MapLayerSequenceId;
use sequence_model::{
    config::{SequenceEndTransition, Wait},
    loaded::WaitSequenceHandle,
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use sprite_model::loaded::SpriteRenderSequenceHandle;

/// Map layer `Component` storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct MapLayerComponentStorages<'s> {
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `Wait` components.
    #[derivative(Debug = "ignore")]
    pub waits: WriteStorage<'s, Wait>,
    /// `MapLayerSequenceId` components.
    #[derivative(Debug = "ignore")]
    pub map_layer_sequence_ids: WriteStorage<'s, MapLayerSequenceId>,
    /// `SequenceEndTransition` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: WriteStorage<'s, SequenceEndTransition<MapLayerSequenceId>>,
    /// `SequenceStatus` components.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
    /// `FrameIndexClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_index_clocks: WriteStorage<'s, FrameIndexClock>,
    /// `FrameWaitClock` components.
    #[derivative(Debug = "ignore")]
    pub frame_wait_clocks: WriteStorage<'s, FrameWaitClock>,
    /// `SpriteRender` components.
    #[derivative(Debug = "ignore")]
    pub sprite_renders: WriteStorage<'s, SpriteRender>,
    /// `WaitSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub wait_sequence_handles: WriteStorage<'s, WaitSequenceHandle>,
    /// `SpriteRenderSequenceHandle` components.
    #[derivative(Debug = "ignore")]
    pub sprite_render_sequence_handles: WriteStorage<'s, SpriteRenderSequenceHandle>,
}
