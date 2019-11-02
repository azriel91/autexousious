use amethyst::{
    core::Transform,
    ecs::{World, WriteStorage},
    renderer::{transparent::Transparent, SpriteRender},
    shred::{ResourceId, SystemData},
};
use derivative::Derivative;
use sequence_model::{
    config::Wait,
    loaded::{SequenceEndTransition, SequenceId, WaitSequenceHandle},
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use sprite_model::loaded::SpriteRenderSequenceHandle;

/// `SpriteSequence` component storages.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct SpriteSequenceComponentStorages<'s> {
    /// `Transparent` components.
    #[derivative(Debug = "ignore")]
    pub transparents: WriteStorage<'s, Transparent>,
    /// `Transform` components.
    #[derivative(Debug = "ignore")]
    pub transforms: WriteStorage<'s, Transform>,
    /// `Wait` components.
    #[derivative(Debug = "ignore")]
    pub waits: WriteStorage<'s, Wait>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `SequenceEndTransition` components.
    #[derivative(Debug = "ignore")]
    pub sequence_end_transitions: WriteStorage<'s, SequenceEndTransition>,
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
