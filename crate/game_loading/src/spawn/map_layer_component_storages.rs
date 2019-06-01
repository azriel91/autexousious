use amethyst::{
    core::Transform,
    ecs::WriteStorage,
    renderer::{transparent::Transparent, SpriteRender},
};
use derivative::Derivative;
use sequence_model::{
    config::{Repeat, Wait},
    loaded::ComponentSequencesHandle,
    play::{FrameIndexClock, FrameWaitClock, SequenceStatus},
};
use shred_derive::SystemData;

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
    /// `Repeat` components.
    #[derivative(Debug = "ignore")]
    pub repeats: WriteStorage<'s, Repeat>,
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
    /// `ComponentSequencesHandle` components.
    #[derivative(Debug = "ignore")]
    pub component_sequences_handles: WriteStorage<'s, ComponentSequencesHandle>,
}
